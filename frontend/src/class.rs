// Class and enumeration specifiers.

use crate::ast::*;
use crate::decl2::Ctx;
use crate::lexer::TK;
use crate::parser::*;
use crate::types::*;

impl P {
    /// Parse `struct|class|union ...`. Returns the type and whether this
    /// was a definition.
    pub fn parse_class_specifier(&mut self) -> PR<(QTy, bool)> {
        let line = self.line();
        let key = match self.tok().text.as_str() {
            "struct" => 0u8,
            "class" => 1,
            _ => 2,
        };
        self.pos += 1;
        self.last_align = 0;
        self.skip_attributes();
        let class_align = self.last_align;
        // name
        let mut name = String::new();
        let mut qual: Option<Result<usize, ()>> = None;
        let mut spec_args = false;
        if self.is_ident() || self.at("::") {
            qual = self.parse_nested_name()?;
            name = self.ident()?;
            if self.at("<") {
                self.skip_template_args()?;
                spec_args = true;
            }
        }
        let is_final = self.at("final") && (self.at_k(1, "{") || self.at_k(1, ":"));
        if is_final {
            self.pos += 1;
        }
        let is_def = self.at("{") || (self.at(":") && !self.at_k(1, ":"));
        if !is_def {
            if name.is_empty() {
                return Err(self.err("anonymous class without a body"));
            }
            let q = self.elaborated_class(key, &name, qual, spec_args, line)?;
            if let Ty::Class(c) = q.t {
                self.classes[c].align = self.classes[c].align.max(class_align);
            }
            return Ok((q, false));
        }
        // definition
        let pending = self.pending_template.take();
        let is_template = pending.is_some() || spec_args || self.in_template();
        let cur = self.decl_scope();
        let mut target_scope = cur;
        let mut cid: Option<usize> = None;
        match qual {
            Some(Ok(s)) => {
                target_scope = s;
                if let Some(ids) = self.scopes[s].names.get(&name).cloned() {
                    for id in ids {
                        if let DK::Class(c) = self.decls[id].k {
                            cid = Some(c);
                        }
                    }
                }
                if cid.is_none() && !spec_args && !self.in_template() {
                    // TYN-7: a nested class defined outside must be declared.
                    self.error(line, "TYN-7", format!("'{}' is not declared in the named scope", name));
                }
            }
            Some(Err(())) => {}
            None => {
                if !spec_args && !name.is_empty() {
                    if let Some(ids) = self.scopes[cur].names.get(&name).cloned() {
                        for id in ids {
                            match &self.decls[id].k {
                                DK::Class(c) => cid = Some(*c),
                                DK::Template(TemplKind::Class(c)) if *c != usize::MAX => cid = Some(*c),
                                DK::Type(_) | DK::Var(_) | DK::Func(_) | DK::Enum(_) | DK::Namespace(_) => {
                                    if !matches!(self.decls[id].k, DK::Var(_) | DK::Func(_)) {
                                        self.error(line, "TYN-2", format!("'{}' already declared as something else", name));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        if spec_args {
            cid = None;
        }
        let c = match cid {
            Some(c) => {
                if self.classes[c].complete {
                    // TYN-7: a class must not be defined two times.
                    self.error(line, "TYN-7", format!("class '{}' defined twice", name));
                }
                if (self.classes[c].key == 2) != (key == 2) {
                    self.error(line, "TYN-4", "class key does not match the earlier declaration");
                }
                c
            }
            None => {
                let parent = if qual.is_some() { target_scope } else { self.cur_scope() };
                let scope = self.new_scope(SK::Class(self.classes.len()), Some(parent), &name);
                self.classes.push(ClassInfo {
                    name: name.clone(),
                    key,
                    scope,
                    complete: false,
                    fields: vec![],
                    bases: vec![],
                    has_conv: false,
                    conv_targets: vec![],
                    ctors: vec![],
                    dtor: None,
                    has_virtual: false,
                    template: is_template,
                    opaque: spec_args,
                    line,
                    align: 0,
                });
                let c = self.classes.len() - 1;
                if !name.is_empty() && !spec_args && !matches!(qual, Some(Err(()))) {
                    let k = if pending.is_some() { DK::Template(TemplKind::Class(c)) } else { DK::Class(c) };
                    self.add_decl_in(target_scope, &name, k, line);
                }
                c
            }
        };
        self.classes[c].template |= is_template;
        self.classes[c].align = self.classes[c].align.max(class_align);
        let class_scope = self.classes[c].scope;
        // bases
        if self.eat(":") {
            loop {
                self.skip_attributes();
                let mut virt = false;
                let mut access = if key == 1 { 2 } else { 0 };
                loop {
                    if self.eat("virtual") {
                        virt = true;
                    } else if self.eat("public") {
                        access = 0;
                    } else if self.eat("protected") {
                        access = 1;
                    } else if self.eat("private") {
                        access = 2;
                    } else {
                        break;
                    }
                }
                let bl = self.line();
                let bq = if self.tok().kind == TK::Ident && crate::decl::simple_type_kw(&self.tok().text) {
                    let t = self.parse_type_id()?;
                    self.error(bl, "DER-1", "base is not a class type");
                    let _ = t;
                    QTy::unknown()
                } else if self.at("decltype") {
                    self.pos += 1;
                    self.parse_decltype()?
                } else if self.qualified_names_type(0) {
                    self.parse_type_name()?
                } else {
                    let n = self.tok().text.clone();
                    self.skip_qualified_name()?;
                    if !self.lookup_uncertain() && !is_template && self.lookup(&n).is_none() {
                        self.error(bl, "DER-1", format!("base '{}' is not a class", n));
                    }
                    QTy::unknown()
                };
                self.eat("...");
                let bc = match bq.t {
                    Ty::Class(b) => {
                        if !self.classes[b].complete {
                            // DRV: a base class must be complete.
                            self.error(bl, "DER-1", "base class is incomplete");
                            None
                        } else {
                            Some(b)
                        }
                    }
                    Ty::Unknown => None,
                    _ => {
                        self.error(bl, "DER-1", "base is not a class type");
                        None
                    }
                };
                if let Some(b) = bc {
                    if self.classes[b].has_virtual {
                        self.classes[c].has_virtual = true;
                    }
                    if self.classes[b].key == 2 || key == 2 {
                        self.error(bl, "DER-1", "a union cannot be a base or have a base");
                    }
                }
                self.classes[c].bases.push(BaseInfo { class: bc, virt, access });
                if !self.eat(",") {
                    break;
                }
            }
        }
        self.expect("{")?;
        let saved = self.stack.clone();
        if let Some(Ok(s)) = qual {
            let _ = s;
        }
        self.stack.push(class_scope);
        // injected class name
        if !name.is_empty() && self.scopes[class_scope].names.get(&name).is_none() {
            self.add_decl_in(class_scope, &name, DK::Class(c), line);
        }
        self.class_depth += 1;
        let saved_templ = self.templ;
        if is_template && self.templ == 0 {
            self.templ = 1;
        }
        while !self.at("}") && self.tok().kind != TK::Eof {
            self.parse_declaration_recover(Ctx::Class(c));
        }
        self.templ = saved_templ;
        self.expect("}")?;
        self.class_depth -= 1;
        self.stack = saved;
        self.classes[c].complete = true;
        self.pending_template = None;
        if self.class_depth == 0 {
            self.run_deferred();
        }
        let _ = pending;
        if is_template {
            return Ok((QTy::unknown(), true));
        }
        Ok((QTy::new(Ty::Class(c)), true))
    }

    fn elaborated_class(&mut self, key: u8, name: &str, qual: Option<Result<usize, ()>>, spec_args: bool, line: u32) -> PR<QTy> {
        if spec_args {
            return Ok(QTy::unknown());
        }
        let decl_only = self.at(";");
        let found = match qual {
            Some(Ok(s)) => self.lookup_in_scope(s, name, true),
            Some(Err(())) => return Ok(QTy::unknown()),
            None => {
                if decl_only {
                    let cur = self.decl_scope();
                    self.scopes[cur].names.get(name).cloned().filter(|v| !v.is_empty())
                } else {
                    self.lookup_tag(name)
                }
            }
        };
        if let Some(ids) = found {
            for id in ids {
                match &self.decls[id].k {
                    DK::Class(c) => {
                        let c = *c;
                        if (self.classes[c].key == 2) != (key == 2) {
                            // TYN-4: union for a class, or the reverse.
                            self.error(line, "TYN-4", "class key does not match the declaration");
                        }
                        return Ok(QTy::new(Ty::Class(c)));
                    }
                    DK::Template(TemplKind::Class(_)) => return Ok(QTy::unknown()),
                    DK::Enum(_) => {
                        self.error(line, "TYN-4", "class key used for an enumeration");
                        return Ok(QTy::unknown());
                    }
                    DK::Type(q) => {
                        if let Ty::Class(c) = q.t {
                            // an alias after a class key is not valid
                            self.error(line, "TYN-4", "elaborated type specifier names a typedef");
                            return Ok(QTy::new(Ty::Class(c)));
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(Ok(_)) = qual {
            if decl_only {
                // TYN-7: a class key and a qualified name alone.
                self.error(line, "TYN-7", "declaration of a qualified class name without a body");
            } else {
                self.error(line, "NAM-1", format!("no class named '{}' in the named scope", name));
            }
            return Ok(QTy::unknown());
        }
        // TYN-4: declare the class in the enclosing namespace or block.
        let mut scope = self.decl_scope();
        if !decl_only {
            for &s in self.stack.iter().rev() {
                match self.scopes[s].kind {
                    SK::Namespace | SK::Block => {
                        scope = s;
                        break;
                    }
                    _ => {}
                }
            }
        }
        let cscope = self.new_scope(SK::Class(self.classes.len()), Some(scope), name);
        self.classes.push(ClassInfo {
            name: name.to_string(),
            key,
            scope: cscope,
            complete: false,
            fields: vec![],
            bases: vec![],
            has_conv: false,
            conv_targets: vec![],
            ctors: vec![],
            dtor: None,
            has_virtual: false,
            template: self.pending_template.is_some(),
            opaque: false,
            line,
            align: 0,
        });
        let c = self.classes.len() - 1;
        let k = if self.pending_template.is_some() { DK::Template(TemplKind::Class(c)) } else { DK::Class(c) };
        self.add_decl_in(scope, name, k, line);
        if self.pending_template.is_some() {
            return Ok(QTy::unknown());
        }
        Ok(QTy::new(Ty::Class(c)))
    }

    /// Lookup that ignores names other than classes and enumerations
    /// (TYN-4).
    pub fn lookup_tag(&self, name: &str) -> Option<Vec<DeclId>> {
        for &s in self.stack.iter().rev() {
            if let Some(v) = self.lookup_in_scope(s, name, true) {
                let tags: Vec<DeclId> = v.into_iter().filter(|&i| matches!(self.decls[i].k, DK::Class(_) | DK::Enum(_) | DK::Template(_) | DK::Type(_))).collect();
                if !tags.is_empty() {
                    return Some(tags);
                }
            }
        }
        None
    }

    pub fn parse_enum_specifier(&mut self, _ds: &crate::decl::DeclSpec) -> PR<(QTy, bool)> {
        let line = self.line();
        self.expect("enum")?;
        let scoped = self.eat("class") || self.eat("struct");
        self.skip_attributes();
        let mut name = String::new();
        let mut qual = None;
        if self.is_ident() || self.at("::") {
            qual = self.parse_nested_name()?;
            name = self.ident()?;
        }
        let mut fixed: Option<Ty> = None;
        if self.at(":") && !self.at_k(1, ":") {
            self.pos += 1;
            let fl = self.line();
            let q = self.parse_type_id()?;
            match &q.t {
                t if t.is_integral() => fixed = Some(t.clone()),
                Ty::Unknown => fixed = Some(Ty::Int),
                Ty::Enum(e) => {
                    let _ = e;
                    self.error(fl, "ENUM-4", "underlying type must be an integer type");
                    fixed = Some(Ty::Int);
                }
                _ => {
                    // ENUM-4: a fixed underlying type must be an integer type.
                    self.error(fl, "ENUM-4", "underlying type must be an integer type");
                    fixed = Some(Ty::Int);
                }
            }
        }
        let is_def = self.at("{");
        let cur = self.decl_scope();
        // Find an earlier declaration.
        let prev = match qual {
            Some(Ok(s)) => self.lookup_in_scope(s, &name, false),
            Some(Err(())) => return Ok((QTy::unknown(), false)),
            None => {
                if is_def || self.at(";") {
                    self.scopes[cur].names.get(&name).cloned().filter(|v| !v.is_empty())
                } else {
                    self.lookup_tag(&name)
                }
            }
        };
        let mut eid = None;
        if !name.is_empty() {
            if let Some(ids) = prev {
                for id in ids {
                    match &self.decls[id].k {
                        DK::Enum(e) => eid = Some(*e),
                        DK::Class(_) => {
                            self.error(line, "TYN-4", "enum key used for a class");
                            return Ok((QTy::unknown(), false));
                        }
                        DK::Type(q) => {
                            if let Ty::Enum(e) = q.t {
                                if !is_def && !self.at(";") {
                                    return Ok((QTy::new(Ty::Enum(e)), false));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if !is_def && !self.at(";") && fixed.is_none() {
            // elaborated reference
            if scoped {
                // TYN-4: `enum class` in a reference to an enumeration.
                self.error(line, "TYN-4", "'enum class' in a reference to an enumeration");
            }
            return match eid {
                Some(e) => Ok((QTy::new(Ty::Enum(e)), false)),
                None => {
                    if self.lookup_uncertain() {
                        return Ok((QTy::unknown(), false));
                    }
                    self.error(line, "TYN-7", format!("unknown enumeration '{}'", name));
                    Ok((QTy::unknown(), false))
                }
            };
        }
        let opaque = !is_def;
        if opaque && !scoped && fixed.is_none() {
            // ENUM-7 / TYN-7: an opaque unscoped declaration needs a fixed type.
            self.error(line, "ENUM-7", "opaque declaration of an unscoped enumeration without a fixed underlying type");
        }
        let e = match eid {
            Some(e) => {
                let ei = self.enums[e].clone();
                if ei.scoped != scoped {
                    self.error(line, "ENUM-7", "enumeration redeclared with a different scope kind");
                }
                let old_fixed = if ei.scoped && ei.fixed.is_none() { Some(Ty::Int) } else { ei.fixed.clone() };
                let new_fixed = if scoped && fixed.is_none() { Some(Ty::Int) } else { fixed.clone() };
                if old_fixed != new_fixed {
                    self.error(line, "ENUM-7", "enumeration redeclared with a different underlying type");
                }
                if is_def && ei.complete {
                    self.error(line, "TYN-7", "enumeration defined twice");
                }
                e
            }
            None => {
                let scope = self.new_scope(SK::Enum(self.enums.len()), Some(self.cur_scope()), &name);
                let underlying = fixed.clone().unwrap_or(Ty::Int);
                self.enums.push(EnumInfo { name: name.clone(), scoped, fixed: if scoped && fixed.is_none() { Some(Ty::Int) } else { fixed.clone() }, underlying, complete: false, enumerators: vec![], scope });
                let e = self.enums.len() - 1;
                if !name.is_empty() {
                    if let Some(prev) = self.scopes[cur].names.get(&name).cloned() {
                        for p in prev {
                            if !matches!(self.decls[p].k, DK::Var(_) | DK::Func(_)) {
                                self.error(line, "NAM-3", format!("'{}' already declared", name));
                            }
                        }
                    }
                    self.add_decl_in(cur, &name, DK::Enum(e), line);
                }
                e
            }
        };
        if !is_def {
            return Ok((QTy::new(Ty::Enum(e)), false));
        }
        self.parse_enumerators(e, line)?;
        Ok((QTy::new(Ty::Enum(e)), true))
    }

    fn parse_enumerators(&mut self, e: usize, _line: u32) -> PR<()> {
        self.expect("{")?;
        let escope0 = self.enums[e].scope;
        self.stack.push(escope0);
        let r = self.parse_enumerators_inner(e);
        self.stack.pop();
        r
    }

    fn parse_enumerators_inner(&mut self, e: usize) -> PR<()> {
        let scoped = self.enums[e].scoped;
        let fixed = self.enums[e].fixed.clone();
        let escope = self.enums[e].scope;
        let outer = self.decl_scope();
        let mut next: i128 = 0;
        let mut prev_overflow = false;
        let mut values = Vec::new();
        while !self.at("}") {
            let line = self.line();
            let name = self.ident()?;
            self.skip_attributes();
            let mut v = next;
            if prev_overflow && !self.at("=") {
                // ENUM-1: an implicit value must fit the underlying type.
                self.error(line, "ENUM-1", "implicit enumerator value does not fit");
            }
            if self.eat("=") {
                // ENUM-12: the name is not in scope in its own value.
                let el = self.line();
                let ex = self.parse_conditional()?;
                let et = nonref(&ex.ty);
                if !(et.t.is_integral() || matches!(et.t, Ty::Enum(_) | Ty::Unknown)) {
                    self.error(el, "CE-11", "enumerator value must have integer or enumeration type");
                }
                match self.const_int(&ex) {
                    Ok(x) => v = x,
                    Err(crate::consteval::CeErr::NotConst(r, m)) => {
                        if !self.in_template() {
                            self.error(el, "ENUM-1", format!("enumerator value is not a constant expression ({}: {})", r, m));
                        }
                    }
                    Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(el, format!("enumerator value: {}", m)),
                }
                if let Some(f) = &fixed {
                    if !fits(v, f) && !ex.ty.is_unknown() {
                        // ENUM-4: each value must fit without narrowing.
                        self.error(el, "ENUM-4", format!("enumerator value {} does not fit the underlying type", v));
                    }
                }
            }
            if let Some(f) = &fixed {
                prev_overflow = v >= f.int_max();
            } else {
                prev_overflow = v >= i64::MAX as i128 && v >= u64::MAX as i128;
            }
            next = v + 1;
            values.push((name.clone(), v));
            // declare
            if self.scopes[escope].names.contains_key(&name) {
                self.error(line, "ENUM-1", format!("enumerator '{}' declared twice", name));
            }
            let ty = if fixed.is_some() || scoped { Ty::Enum(e) } else { Ty::Enum(e) };
            let _ = ty;
            let id = self.add_decl_in(escope, &name, DK::Enumerator(e, v), line);
            if !scoped {
                // ENUM-2: unscoped enumerators go in the enclosing scope.
                if let Some(prev) = self.scopes[outer].names.get(&name) {
                    if !prev.is_empty() {
                        self.error(line, "ENUM-2", format!("enumerator '{}' conflicts with an earlier declaration", name));
                    }
                }
                self.scopes[outer].names.entry(name.clone()).or_default().push(id);
            }
            if !self.eat(",") {
                break;
            }
        }
        self.expect("}")?;
        // ENUM-5: choose the underlying type of an unscoped enumeration
        // without a fixed type.
        if fixed.is_none() {
            let min = values.iter().map(|x| x.1).min().unwrap_or(0);
            let max = values.iter().map(|x| x.1).max().unwrap_or(0);
            let cands = [Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong];
            let mut chosen = None;
            for c in cands {
                if fits(min, &c) && fits(max, &c) {
                    chosen = Some(c);
                    break;
                }
            }
            match chosen {
                Some(c) => self.enums[e].underlying = c,
                None => {
                    self.error(self.line(), "ENUM-5", "no integer type can hold all enumerator values");
                }
            }
        } else if let Some(f) = &fixed {
            if scoped {
                for (_, v) in &values {
                    if !fits(*v, f) {
                        self.error(self.line(), "ENUM-4", "enumerator value does not fit in int");
                        break;
                    }
                }
            }
        }
        self.enums[e].enumerators = values;
        self.enums[e].complete = true;
        Ok(())
    }

    /// ENUM-6: the range of values of an enumeration without a fixed type.
    pub fn enum_range(&self, e: usize) -> (i128, i128) {
        let ei = &self.enums[e];
        if let Some(f) = &ei.fixed {
            return (f.int_min(), f.int_max());
        }
        let min = ei.enumerators.iter().map(|x| x.1).min().unwrap_or(0);
        let max = ei.enumerators.iter().map(|x| x.1).max().unwrap_or(0);
        if ei.enumerators.is_empty() {
            return (0, 1);
        }
        // smallest bit-field that holds all values
        let mut bits = 1u32;
        loop {
            let (lo, hi) = if min < 0 { (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1) } else { (0, (1i128 << bits) - 1) };
            if min >= lo && max <= hi {
                return (lo, hi);
            }
            bits += 1;
            if bits > 64 {
                return (min, max);
            }
        }
    }

    pub fn expr_is_type_unknown(&self, e: &Expr) -> bool {
        e.ty.is_unknown()
    }
}
