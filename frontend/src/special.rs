// Trivial special member functions (CTOR-23, DTOR-9, CPY-27) and the
// special members of a union that UNI-3 deletes.

use crate::ast::DeclId;
use crate::parser::*;
use crate::types::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Sm {
    Default,
    Dtor,
    CopyCtor,
    MoveCtor,
    CopyAssign,
    MoveAssign,
}

impl Sm {
    pub fn what(self) -> &'static str {
        match self {
            Sm::Default => "default constructor",
            Sm::Dtor => "destructor",
            Sm::CopyCtor => "copy constructor",
            Sm::MoveCtor => "move constructor",
            Sm::CopyAssign => "copy assignment operator",
            Sm::MoveAssign => "move assignment operator",
        }
    }
}

/// What a class declares for one special member.
enum Declared {
    /// Nothing: the function is implicit.
    None,
    /// A function whose first declaration has `= default`.
    Defaulted,
    /// A function the program provides (or defaults after its first
    /// declaration).
    Provided,
    Deleted,
    /// The front end cannot tell.
    Unknown,
}

impl P {
    fn class_of_elem(t: &Ty) -> Option<usize> {
        match t {
            Ty::Array(e, _) => Self::class_of_elem(&e.t),
            Ty::Class(c) => Some(*c),
            _ => None,
        }
    }

    /// The kind of the first parameter of a copy or move function of `c`:
    /// Some(false) for an lvalue reference or a value, Some(true) for an
    /// rvalue reference.
    fn copy_param(&self, f: DeclId, c: usize, by_value: bool) -> Option<bool> {
        let fi = self.func(f)?;
        let first = if fi.explicit_obj { 1 } else { 0 };
        if fi.params.len() <= first || fi.params.len() - first - 1 > fi.defaults {
            return None;
        }
        let ty = &self.var(fi.params[first])?.ty;
        match &ty.t {
            Ty::LRef(x) if x.t == Ty::Class(c) => Some(false),
            Ty::RRef(x) if x.t == Ty::Class(c) => Some(true),
            Ty::Class(x) if by_value && *x == c => Some(false),
            _ => None,
        }
    }

    fn declared(&self, c: usize, sm: Sm) -> Declared {
        let ci = &self.classes[c];
        let cands: Vec<DeclId> = match sm {
            Sm::Default => {
                if ci.ctors.is_empty() {
                    return Declared::None;
                }
                match self.default_ctor(c) {
                    Some(d) => vec![d],
                    None => return Declared::Unknown,
                }
            }
            Sm::Dtor => ci.dtor.into_iter().collect(),
            Sm::CopyCtor | Sm::MoveCtor => {
                let want = sm == Sm::MoveCtor;
                if ci.ctors.iter().any(|&i| self.func(i).is_none_or(|f| f.template)) {
                    return Declared::Unknown;
                }
                ci.ctors.iter().copied().filter(|&i| self.copy_param(i, c, false) == Some(want)).collect()
            }
            Sm::CopyAssign | Sm::MoveAssign => {
                let want = sm == Sm::MoveAssign;
                let Some(ids) = self.scopes[ci.scope].names.get("operator=") else { return Declared::None };
                if ids.iter().any(|&i| self.func(i).is_none_or(|f| f.template)) {
                    return Declared::Unknown;
                }
                ids.iter().copied().filter(|&i| self.copy_param(i, c, true) == Some(want)).collect()
            }
        };
        match cands.as_slice() {
            [] => Declared::None,
            [d] => {
                let f = self.func(*d).unwrap();
                if f.deleted {
                    Declared::Deleted
                } else if f.first_defaulted {
                    Declared::Defaulted
                } else {
                    Declared::Provided
                }
            }
            _ => Declared::Unknown,
        }
    }

    /// Whether a special member of a class is trivial (CTOR-23, DTOR-9,
    /// CPY-27). None when the front end cannot tell.
    pub fn trivial(&self, c: usize, sm: Sm) -> Option<bool> {
        self.trivial_d(c, sm, 0)
    }

    fn trivial_d(&self, c: usize, sm: Sm, depth: u32) -> Option<bool> {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || depth > 16 {
            return None;
        }
        let mut sm = sm;
        match self.declared(c, sm) {
            Declared::Provided => return Some(false),
            Declared::Unknown => return None,
            Declared::Deleted | Declared::Defaulted => {}
            Declared::None => {
                // With no move function declared, a move uses the copy
                // function.
                let copy = match sm {
                    Sm::MoveCtor => Some(Sm::CopyCtor),
                    Sm::MoveAssign => Some(Sm::CopyAssign),
                    _ => None,
                };
                if let Some(cp) = copy {
                    match self.declared(c, cp) {
                        Declared::Provided => return Some(false),
                        Declared::Unknown => return None,
                        _ => sm = cp,
                    }
                }
            }
        }
        if sm == Sm::Dtor {
            if ci.dtor.and_then(|d| self.func(d)).is_some_and(|f| f.is_virtual) {
                return Some(false);
            }
        } else if ci.has_virtual || ci.bases.iter().any(|b| b.virt) {
            return Some(false);
        }
        for b in &ci.bases {
            if !self.trivial_d(b.class?, sm, depth + 1)? {
                return Some(false);
            }
        }
        for &fd in &ci.fields {
            let DK::Field(fi) = &self.decls[fd].k else { continue };
            if sm == Sm::Default && fi.init.is_some() {
                return Some(false);
            }
            if let Some(m) = Self::class_of_elem(&fi.ty.t) {
                if !self.trivial_d(m, sm, depth + 1)? {
                    return Some(false);
                }
            }
        }
        Some(true)
    }

    /// Whether UNI-3 deletes a special member of a class: the class is a
    /// union, or has an anonymous union member, or has a base or member
    /// whose special member UNI-3 deletes, and the class does not declare
    /// that function itself.
    pub fn uni3_deleted(&self, c: usize, sm: Sm) -> bool {
        self.uni3_d(c, sm, 0)
    }

    fn uni3_d(&self, c: usize, sm: Sm, depth: u32) -> bool {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || depth > 16 {
            return false;
        }
        match self.declared(c, sm) {
            Declared::None => {}
            Declared::Defaulted => {}
            _ => return false,
        }
        if sm == Sm::MoveCtor || sm == Sm::MoveAssign {
            // A class that declares a copy function and no move function
            // moves by copying.
            let cp = if sm == Sm::MoveCtor { Sm::CopyCtor } else { Sm::CopyAssign };
            if matches!(self.declared(c, sm), Declared::None) && !matches!(self.declared(c, cp), Declared::None) {
                return self.uni3_d(c, cp, depth);
            }
        }
        let variant = |u: usize| -> bool {
            let ui = &self.classes[u];
            if sm == Sm::Default && ui.fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(x) if x.init.is_some())) {
                return false;
            }
            ui.fields.iter().any(|&f| {
                let DK::Field(fi) = &self.decls[f].k else { return false };
                Self::class_of_elem(&fi.ty.t).is_some_and(|m| self.trivial(m, sm) == Some(false))
            })
        };
        if ci.key == 2 && variant(c) {
            return true;
        }
        if ci.anon_unions.iter().any(|&u| variant(u)) {
            return true;
        }
        let anon_fields: Vec<DeclId> = ci.anon_unions.iter().flat_map(|&u| self.classes[u].fields.iter().map(|&f| self.decls[f].name.clone())).filter_map(|n| self.scopes[ci.scope].names.get(&n).and_then(|v| v.first().copied())).collect();
        for b in &ci.bases {
            if b.class.is_some_and(|bc| self.uni3_d(bc, sm, depth + 1)) {
                return true;
            }
        }
        for &fd in &ci.fields {
            if anon_fields.contains(&fd) {
                continue;
            }
            let DK::Field(fi) = &self.decls[fd].k else { continue };
            if sm == Sm::Default && fi.init.is_some() {
                continue;
            }
            if fi.ty.t.is_ref() {
                continue;
            }
            if let Some(m) = Self::class_of_elem(&fi.ty.t) {
                if self.uni3_d(m, sm, depth + 1) {
                    return true;
                }
            }
        }
        false
    }

    /// JMP-6, SW-5: whether a jump can skip the declaration of a variable
    /// of this type that has no initializer. None when the front end cannot
    /// tell.
    pub fn skippable_type(&self, t: &Ty) -> Option<bool> {
        match t {
            Ty::Array(e, _) => self.skippable_type(&e.t),
            Ty::Class(c) => Some(self.trivial(*c, Sm::Default)? && self.trivial(*c, Sm::Dtor)?),
            Ty::Unknown => None,
            _ => Some(true),
        }
    }

    /// Whether the destructor of a class is deleted: declared deleted, or
    /// implicit or defaulted with a base or member whose destructor is
    /// deleted (DTOR-3), or deleted by UNI-3.
    pub fn dtor_deleted(&self, c: usize, depth: u32) -> bool {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || depth > 16 {
            return false;
        }
        match self.declared(c, Sm::Dtor) {
            Declared::Deleted => return true,
            Declared::Provided | Declared::Unknown => return false,
            _ => {}
        }
        if self.uni3_deleted(c, Sm::Dtor) {
            return true;
        }
        if ci.bases.iter().any(|b| b.class.is_some_and(|bc| self.dtor_deleted(bc, depth + 1))) {
            return true;
        }
        if ci.key == 2 {
            return false;
        }
        ci.fields.iter().any(|&f| match &self.decls[f].k {
            DK::Field(fi) if !fi.ty.t.is_ref() => Self::class_of_elem(&fi.ty.t).is_some_and(|m| self.dtor_deleted(m, depth + 1)),
            _ => false,
        })
    }

    /// Checks for a default initialization of an object of class `c`:
    /// CTOR-19 (no default constructor), CTOR-20 (CTOR-3 deletes it), and
    /// UNI-11 (UNI-3 deletes it).
    pub fn check_default_init(&mut self, c: usize, line: u32) {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || self.class_uncertain(c) {
            return;
        }
        let name = ci.name.clone();
        if !ci.ctors.is_empty() && ci.ctors.iter().all(|&i| self.func(i).is_some_and(|f| !f.template)) {
            let any_default = ci.ctors.iter().any(|&i| self.func(i).is_some_and(|f| f.params.len() <= f.defaults));
            if !any_default {
                self.error(line, "CTOR-19", format!("'{}' has no default constructor", name));
                return;
            }
        }
        if self.default_ctor_deleted(c, 0) == Some(true) {
            self.error(line, "CTOR-20", format!("default constructor of '{}' is deleted", name));
        } else if self.uni3_deleted(c, Sm::Default) {
            self.error(line, "UNI-11", format!("the default constructor of '{}' is deleted (UNI-3)", name));
        }
    }

    /// CTOR-4: whether a const object of class `c` can have no
    /// initializer. None when the front end cannot tell.
    pub fn const_default_ok(&self, c: usize, depth: u32) -> Option<bool> {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || self.class_uncertain(c) || depth > 16 || ci.key == 2 {
            return None;
        }
        match self.declared(c, Sm::Default) {
            Declared::Provided => return Some(true),
            Declared::Unknown => return None,
            _ => {}
        }
        for b in &ci.bases {
            if !self.const_default_ok(b.class?, depth + 1)? {
                return Some(false);
            }
        }
        for &f in &ci.fields {
            let DK::Field(fi) = &self.decls[f].k else { continue };
            if fi.init.is_some() || fi.bits == Some(0) {
                continue;
            }
            match Self::class_of_elem(&fi.ty.t) {
                Some(m) => {
                    if !self.const_default_ok(m, depth + 1)? {
                        return Some(false);
                    }
                }
                None => return Some(false),
            }
        }
        Some(true)
    }

    /// The user-declared copy (`rvalue` false) or move (`rvalue` true)
    /// constructors or assignment operators of a class (CPY-1 to CPY-3).
    /// None when a template could take part.
    fn copy_cands(&self, c: usize, assign: bool, rvalue: bool) -> Option<Vec<DeclId>> {
        let ci = &self.classes[c];
        let ids: Vec<DeclId> = if assign {
            self.scopes[ci.scope].names.get("operator=").cloned().unwrap_or_default()
        } else {
            ci.ctors.clone()
        };
        if ids.iter().any(|&i| self.func(i).is_none_or(|f| f.template)) {
            return None;
        }
        Some(ids.into_iter().filter(|&i| self.copy_param(i, c, assign) == Some(rvalue)).collect())
    }

    /// Whether the first parameter of a copy function takes a `const`
    /// object.
    fn takes_const(&self, f: DeclId) -> bool {
        let Some(fi) = self.func(f) else { return true };
        let first = if fi.explicit_obj { 1 } else { 0 };
        match fi.params.get(first).and_then(|&p| self.var(p)).map(|v| &v.ty.t) {
            Some(Ty::LRef(x)) => x.c,
            Some(Ty::Class(_)) => true,
            _ => true,
        }
    }

    /// CPY-7: whether an implicit or defaulted copy or move function of a
    /// class is deleted.
    pub fn cpy7_deleted(&self, c: usize, assign: bool, rvalue: bool, depth: u32) -> Option<bool> {
        let ci = &self.classes[c];
        if ci.key == 2 {
            return Some(false);
        }
        let mut parts: Vec<(QTy, bool)> = ci.bases.iter().map(|b| b.class.map(|bc| (QTy::new(Ty::Class(bc)), true))).collect::<Option<Vec<_>>>()?;
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.decls[f].k {
                parts.push((fi.ty.clone(), false));
            }
        }
        for (t, _is_base) in parts {
            if !assign && !rvalue && matches!(t.t, Ty::RRef(_)) {
                return Some(true);
            }
            if assign && (t.t.is_ref() || (t.c && !matches!(t.t, Ty::Array(..)))) {
                return Some(true);
            }
            if t.t.is_ref() {
                continue;
            }
            if let Some(m) = Self::class_of_elem(&t.t) {
                if self.dtor_deleted(m, depth + 1) {
                    return Some(true);
                }
                // CPY-7: a function of the base or member that is not
                // accessible.
                if self.classes[m].dtor.is_some_and(|d| self.sub_inaccessible(m, d, _is_base)) {
                    return Some(true);
                }
                if let Some(cands) = self.copy_cands(m, assign, rvalue) {
                    if !cands.is_empty() && cands.iter().all(|&f| self.sub_inaccessible(m, f, _is_base)) {
                        return Some(true);
                    }
                }
                let st = if rvalue { self.rv_status(m, assign, depth + 1) } else { self.lv_status(m, assign, depth + 1) };
                match st {
                    CopySt::Deleted(_) => return Some(true),
                    CopySt::Unknown => return None,
                    CopySt::Ok { .. } => {}
                }
            }
        }
        Some(false)
    }

    /// How an lvalue of class `c` is copied (CPY-4, CPY-6, CPY-7, CPY-14).
    pub fn lv_status(&self, c: usize, assign: bool, depth: u32) -> CopySt {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || depth > 16 || self.class_uncertain(c) {
            return CopySt::Unknown;
        }
        let Some(cands) = self.copy_cands(c, assign, false) else { return CopySt::Unknown };
        if !cands.is_empty() {
            let mut ok = Vec::new();
            let mut why = "DEL-1";
            for &f in &cands {
                let fi = self.func(f).unwrap();
                if fi.deleted {
                    continue;
                }
                if fi.first_defaulted {
                    match self.cpy7_deleted(c, assign, false, depth) {
                        Some(true) => {
                            why = "CPY-23";
                            continue;
                        }
                        None => return CopySt::Unknown,
                        _ => {}
                    }
                    // CPY-14: a defaulted copy constructor takes `T&` or
                    // the parameter of the implicit one.
                    if !assign {
                        let first = &self.var(fi.params[0]).unwrap().ty.t;
                        if let Ty::LRef(x) = first {
                            if x.v {
                                why = "CPY-26";
                                continue;
                            }
                            if x.c && !self.members_copy_const(c, assign, depth) {
                                why = "CPY-26";
                                continue;
                            }
                        }
                    }
                }
                ok.push(f);
            }
            if ok.is_empty() {
                return CopySt::Deleted(why);
            }
            let const_src = ok.iter().any(|&f| self.takes_const(f));
            let explicit = ok.iter().all(|&f| self.func(f).is_some_and(|x| x.explicit));
            return CopySt::Ok { const_src, explicit, implicit: false };
        }
        // CPY-4: a user-declared move function deletes the implicit copy.
        match self.copy_cands(c, false, true).zip(self.copy_cands(c, true, true)) {
            None => return CopySt::Unknown,
            Some((a, b)) if !a.is_empty() || !b.is_empty() => return CopySt::Deleted("CPY-21"),
            _ => {}
        }
        match self.cpy7_deleted(c, assign, false, depth) {
            Some(true) => return CopySt::Deleted("CPY-23"),
            None => return CopySt::Unknown,
            _ => {}
        }
        CopySt::Ok { const_src: self.members_copy_const(c, assign, depth), explicit: false, implicit: true }
    }

    /// CPY-6: whether each base and member can be copied from a `const`
    /// object.
    fn members_copy_const(&self, c: usize, assign: bool, depth: u32) -> bool {
        let ci = &self.classes[c];
        let mut classes: Vec<usize> = ci.bases.iter().filter_map(|b| b.class).collect();
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.decls[f].k {
                if !fi.ty.t.is_ref() {
                    if let Some(m) = Self::class_of_elem(&fi.ty.t) {
                        classes.push(m);
                    }
                }
            }
        }
        classes.into_iter().all(|m| !matches!(self.lv_status(m, assign, depth + 1), CopySt::Ok { const_src: false, .. }))
    }

    /// How an xvalue of class `c` initializes or is assigned (CPY-5,
    /// CPY-7): by a move function, or by the copy function.
    pub fn rv_status(&self, c: usize, assign: bool, depth: u32) -> CopySt {
        let ci = &self.classes[c];
        if ci.template || !ci.complete || depth > 16 || self.class_uncertain(c) {
            return CopySt::Unknown;
        }
        let Some(cands) = self.copy_cands(c, assign, true) else { return CopySt::Unknown };
        if !cands.is_empty() {
            let mut any_ok = false;
            for &f in &cands {
                let fi = self.func(f).unwrap();
                if fi.deleted {
                    // CPY-7: an explicitly deleted move takes part.
                    return CopySt::Deleted("CPY-23");
                }
                if fi.first_defaulted {
                    // CPY-14: a defaulted move takes a `T&&` without cv.
                    let first = if fi.explicit_obj { 1 } else { 0 };
                    if let Some(Ty::RRef(x)) = fi.params.get(first).and_then(|&p| self.var(p)).map(|v| &v.ty.t) {
                        if (x.c || x.v) && !assign {
                            return CopySt::Deleted("CPY-26");
                        }
                    }
                    match self.cpy7_deleted(c, assign, true, depth) {
                        Some(true) => continue,
                        None => return CopySt::Unknown,
                        _ => {}
                    }
                }
                any_ok = true;
            }
            if any_ok {
                return CopySt::Ok { const_src: true, explicit: false, implicit: false };
            }
            return self.lv_status(c, assign, depth);
        }
        // CPY-5: the implicit move functions.
        let (Some(cc), Some(ca), Some(mc), Some(ma)) = (self.copy_cands(c, false, false), self.copy_cands(c, true, false), self.copy_cands(c, false, true), self.copy_cands(c, true, true)) else {
            return CopySt::Unknown;
        };
        let has_move = cc.is_empty() && ca.is_empty() && mc.is_empty() && ma.is_empty() && ci.dtor.is_none();
        if has_move {
            match self.cpy7_deleted(c, assign, true, depth) {
                Some(false) => return CopySt::Ok { const_src: true, explicit: false, implicit: true },
                None => return CopySt::Unknown,
                _ => {}
            }
        }
        match self.lv_status(c, assign, depth) {
            CopySt::Ok { explicit, implicit, .. } => CopySt::Ok { const_src: true, explicit, implicit },
            x => x,
        }
    }

    /// Checks for a copy or move of a glvalue `x` of class `c`, in an
    /// initialization (`copy_init` for copy-initialization) or an
    /// assignment: CPY-20 to CPY-24, CPY-26, DEL-1, and UNI-11.
    pub fn check_copy_from(&mut self, c: usize, x: &crate::ast::Expr, copy_init: bool, assign: bool, line: u32) {
        use crate::ast::Cat;
        if x.cat == Cat::PR || x.ty.t != Ty::Class(c) || self.in_template() {
            return;
        }
        let rv = x.cat == Cat::X;
        let sm = match (assign, rv) {
            (false, false) => Sm::CopyCtor,
            (false, true) => Sm::MoveCtor,
            (true, false) => Sm::CopyAssign,
            (true, true) => Sm::MoveAssign,
        };
        let name = self.classes[c].name.clone();
        if self.uni3_deleted(c, sm) {
            self.error(line, "UNI-11", format!("the {} of '{}' is deleted (UNI-3)", sm.what(), name));
            return;
        }
        let st = if rv { self.rv_status(c, assign, 0) } else { self.lv_status(c, assign, 0) };
        match st {
            CopySt::Unknown => {}
            CopySt::Deleted(r) => {
                let msg = format!("the {} of '{}' is deleted", sm.what(), name);
                if !assign && !rv {
                    // CPY-24: an lvalue needs a usable copy constructor.
                    self.error_because(line, r, "CPY-24", msg);
                } else {
                    self.error(line, r, msg);
                }
            }
            CopySt::Ok { const_src, explicit, implicit } => {
                if implicit && self.wflag(&["-Wextra", "-Wdeprecated-copy", "-Wdeprecated"]) {
                    // CPY-13: an implicit copy function in a class with a
                    // user-provided copy function.
                    let other = |p: &P, a: bool| p.copy_cands(c, a, false).is_some_and(|v| v.iter().any(|&f| p.func(f).is_some_and(|fi| !fi.first_defaulted && !fi.deleted)));
                    if other(self, !assign) {
                        self.warn(line, "CPY-13", format!("the implicit {} of '{}' is deprecated, because the class has a user-provided copy function", sm.what(), name));
                    }
                }
                if !rv && !const_src && x.ty.c {
                    let r = if implicit { "CPY-22" } else { "CPY-20" };
                    self.error(line, r, format!("the {} of '{}' cannot copy a const object", sm.what(), name));
                } else if explicit && copy_init && !assign {
                    self.error(line, "CPY-24", format!("the explicit copy constructor of '{}' in copy-initialization", name));
                }
            }
        }
    }

    /// Declaration checks for a copy or move function: CPY-15 and CPY-17.
    /// `fid` is a constructor or an `operator=` of class `c`, and
    /// `defaulted` says whether this declaration has `= default`.
    pub fn check_copy_decl(&mut self, fid: DeclId, c: usize, assign: bool, defaulted: bool, first_decl: bool, line: u32) {
        if self.in_template() {
            return;
        }
        let Some(fi) = self.func(fid) else { return };
        let first = if fi.explicit_obj { 1 } else { 0 };
        let Some(pt) = fi.params.get(first).and_then(|&p| self.var(p)).map(|v| v.ty.clone()) else { return };
        let rest_default = fi.params.len() - first - 1 <= fi.defaults;
        let ret = fi.fty.ret.clone();
        if !assign && pt.t == Ty::Class(c) && rest_default {
            // CPY-15: no constructor that takes its own class by value.
            self.error(line, "CPY-15", format!("constructor of '{}' takes its own class by value", self.classes[c].name));
            return;
        }
        let kind = self.copy_param(fid, c, assign);
        if !defaulted || kind.is_none() {
            return;
        }
        if assign {
            if !matches!(pt.t, Ty::LRef(_) | Ty::RRef(_)) {
                self.error(line, "CPY-17", "a defaulted assignment operator must take a reference");
            } else if !matches!(&ret.t, Ty::LRef(x) if x.t == Ty::Class(c) && !x.c && !x.v) && !ret.is_unknown() {
                self.error(line, "CPY-17", format!("a defaulted assignment operator must return '{}&'", self.classes[c].name));
            }
        }
        if !first_decl && self.classes[c].complete && self.cpy7_deleted(c, assign, kind == Some(true), 0) == Some(true) {
            // CPY-17: a function defaulted after its class must not be one
            // that would be deleted.
            self.error(line, "CPY-17", "a function defaulted after its class would be deleted");
        }
    }

    /// Whether a special member of class `m`, used for a base (`base`) or
    /// a member subobject of another class, is not accessible there. The
    /// other class is not a friend of `m` unless `m` declares a friend.
    pub fn sub_inaccessible(&self, m: usize, f: DeclId, base: bool) -> bool {
        if self.friend_classes.contains(&m) {
            return false;
        }
        let a = self.member_access.get(&f).copied().unwrap_or(0);
        a == 2 || (a == 1 && !base)
    }

    /// DTOR-3: the destructor is deleted or not accessible here.
    pub fn dtor_unusable(&self, c: usize) -> bool {
        self.dtor_deleted(c, 0) || self.classes[c].dtor.is_some_and(|d| !self.accessible(d, false))
    }

    /// CTOR-3: a base or member whose default constructor or destructor
    /// is not accessible to the implicit default constructor.
    pub fn default_parts_inaccessible(&self, c: usize) -> bool {
        let ci = &self.classes[c];
        let mut parts: Vec<(usize, bool)> = ci.bases.iter().filter_map(|b| b.class.map(|bc| (bc, true))).collect();
        if ci.key != 2 {
            for &f in &ci.fields {
                if let DK::Field(fi) = &self.decls[f].k {
                    if fi.ty.t.is_ref() {
                        continue;
                    }
                    if let Some(m) = Self::class_of_elem(&fi.ty.t) {
                        if fi.init.is_none() {
                            parts.push((m, false));
                        } else if self.classes[m].dtor.is_some_and(|d| self.sub_inaccessible(m, d, false)) {
                            return true;
                        }
                    }
                }
            }
        }
        parts.into_iter().any(|(m, base)| {
            let d = !self.classes[m].ctors.is_empty() && self.default_ctor(m).is_some_and(|d| self.sub_inaccessible(m, d, base));
            d || self.classes[m].dtor.is_some_and(|x| self.sub_inaccessible(m, x, base))
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CopySt {
    Ok { const_src: bool, explicit: bool, implicit: bool },
    Deleted(&'static str),
    Unknown,
}
