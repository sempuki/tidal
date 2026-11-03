// Implicit conversion sequences and their comparison
// (specification/foundations/conversion-sequences.md, ICS-1 to ICS-45), and
// overload resolution for calls (specification/foundations/overload-resolution.md,
// OVL-22 to OVL-25, OVL-34 to OVL-36).
//
// A sequence is formed for one argument and one parameter. Two sequences
// for the same argument compare by the steps of ICS-P5. `ics_select`
// finds the viable functions of a call and the one that is better than
// each other one (OVL-25, OVL-34).
//
// Forming returns Err(()) when the front end cannot model the argument
// or the parameter. The caller then falls back to the older ranking.

use crate::ast::*;
use crate::parser::*;
use crate::types::*;

/// The conversion of group 2 of ICS-P2, with what the tie-breakers need.
#[derive(Clone, Debug, PartialEq)]
pub enum Sec {
    None,
    /// An integer promotion or float to double.
    Promo,
    /// An unscoped enumeration with a fixed underlying type to that type
    /// (ICS-34).
    PromoUnderlying,
    /// Any other arithmetic conversion.
    Arith,
    /// A pointer or member pointer to bool (ICS-32).
    PtrBool,
    /// A pointer to an object type to `void*`, with the source pointee.
    PtrVoid(Ty),
    /// A pointer to class `from` to a pointer to its base `to`.
    PtrBase(usize, usize),
    /// A class value or reference of `from` to its base `to`.
    ClassBase(usize, usize),
    /// A member pointer of base `from` to one of derived `to`.
    MemBase(usize, usize),
    /// A null pointer constant to a pointer or member pointer.
    Null,
}

#[derive(Clone, Debug)]
pub struct RefB {
    pub rref: bool,
    /// The reference binds to an rvalue (ICS-28).
    pub to_rvalue: bool,
    /// The reference binds to a function lvalue (ICS-29).
    pub fn_lvalue: bool,
    /// The referenced type with its cv-qualifiers (ICS-30).
    pub referenced: QTy,
    /// The implicit object parameter of a function without a
    /// ref-qualifier (ICS-28 does not apply).
    pub obj_noref: bool,
}

#[derive(Clone, Debug)]
pub struct Std {
    pub rank: u8,
    pub sec: Sec,
    /// A qualification adjustment (group 3).
    pub qual: bool,
    /// The type after the sequence.
    pub to: QTy,
    pub refb: Option<RefB>,
}

impl Std {
    fn identity(to: QTy) -> Std {
        Std { rank: 0, sec: Sec::None, qual: false, to, refb: None }
    }
    fn is_identity(&self) -> bool {
        self.sec == Sec::None && !self.qual
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IK {
    Std,
    User,
    Ambig,
    Ellipsis,
    /// The object slot of a static member function (ICS-14).
    Slot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UKey {
    Fn(DeclId),
    Agg(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LT {
    No,
    InitList,
    /// A reference to an array: element type, bound, number of elements
    /// in the list.
    Array(Ty, Option<u64>, u64),
}

#[derive(Clone, Debug)]
pub struct Ics {
    pub kind: IK,
    /// The standard sequence, or the third part of a user-defined one.
    pub s: Std,
    pub user: Option<UKey>,
    pub list: LT,
}

impl Ics {
    fn std(s: Std) -> Ics {
        Ics { kind: IK::Std, s, user: None, list: LT::No }
    }
    fn ambig() -> Ics {
        Ics { kind: IK::Ambig, s: Std::identity(QTy::unknown()), user: None, list: LT::No }
    }
    pub fn ellipsis() -> Ics {
        Ics { kind: IK::Ellipsis, s: Std::identity(QTy::unknown()), user: None, list: LT::No }
    }
    pub fn slot() -> Ics {
        Ics { kind: IK::Slot, s: Std::identity(QTy::unknown()), user: None, list: LT::No }
    }
}

pub type R = Result<Option<Ics>, ()>;

fn is_list(a: &Expr) -> bool {
    matches!(a.k, EK::InitList(_)) && a.ty.is_unknown()
}

/// How a candidate for a user-defined conversion compares: the sequence
/// to its first parameter or object parameter, and the third part.
struct UCand {
    f: DeclId,
    first: Ics,
    third: Std,
}

impl P {
    /// A user-defined conversion of class object `e` by one of its
    /// conversion functions to `to` (ICS-4, ICS-37). None when the front
    /// end finds no unique function.
    pub fn user_conv(&mut self, e: &Expr, to: &QTy, line: u32) -> Option<Expr> {
        let Ty::Class(ac) = nonref(&e.ty).t else { return None };
        if self.in_template() || !self.class_ok(ac) || to.is_unknown() {
            return None;
        }
        let r = match &to.t {
            Ty::LRef(i) | Ty::RRef(i) => {
                if matches!(i.t, Ty::Func(_)) {
                    // The evaluator does not model a reference to a function
                    // from a conversion function.
                    return None;
                }
                let i = (**i).clone();
                self.ics_user_from_class(e, ac, &i, Some(matches!(to.t, Ty::RRef(_))))
            }
            Ty::Class(_) => return None,
            _ => self.ics_user_from_class(e, ac, &to.unqual(), None),
        };
        let Ok(Some(Ics { kind: IK::User, user: Some(UKey::Fn(fid)), .. })) = r else { return None };
        let f = self.func(fid)?.clone();
        if f.deleted {
            // ICS-8: the chosen conversion function is deleted.
            self.error_because(line, "ICS-8", "DEL-1", format!("conversion by the deleted function '{}'", self.decls[fid].name));
        }
        if self.in_unevaluated == 0 {
            self.use_fn(fid);
        }
        let f = self.func(fid)?.clone();
        let (t, cat) = crate::expr2::ret_cat(&f.fty.ret);
        let call = Expr::new(EK::Call(Box::new(e.clone()), vec![], Some(fid)), t, cat, line);
        if self.in_unevaluated == 0 && self.internal_fns.contains(&fid) {
            self.internal_uses.push((fid, line));
        }
        Some(match &to.t {
            Ty::LRef(_) | Ty::RRef(_) => self.bind_ref(call, to, line, "INIT-7"),
            _ => match self.implicit(call, &to.unqual(), false) {
                Ok(x) => x,
                Err(_) => return None,
            },
        })
    }

    /// The element type when `c` is the class `std::initializer_list<E>`.
    pub fn init_list_elem(&self, c: usize) -> Option<QTy> {
        self.init_list_classes.iter().find(|x| x.1 == c).map(|x| x.0.clone())
    }

    /// ICS-1 to ICS-20: the sequence from argument `a` to parameter `p`.
    /// `std_only` is ICS-10. None: no sequence.
    pub fn ics_form(&mut self, a: &Expr, p: &QTy, std_only: bool) -> R {
        if p.is_unknown() || self.in_template() {
            return Err(());
        }
        // INST-3: the front end makes the classes complete.
        self.require_pointee(p, a.line);
        self.require_pointee(&a.ty, a.line);
        if matches!(a.k, EK::InitList(_)) && a.ty.is_unknown() {
            return self.ics_list(a, p, std_only);
        }
        if let Some((fs, _)) = Self::overload_set_of(a) {
            // OVL-39: each function of the set is tried with the parameter
            // as the target type of OVL-18. The match is the identity.
            let target = match &p.t {
                Ty::Ptr(_) | Ty::MemPtr(..) => p.unqual(),
                _ => p.clone(),
            };
            let m = self.overload_matches(&fs, &target)?;
            return match m.len() {
                0 => Ok(None),
                1 => {
                    // ICS-47: dropping noexcept is a function pointer
                    // conversion, which the identity beats.
                    let dropped = m[0].1.noexcept && matches!(&target.t, Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) if matches!(&x.t, Ty::Func(ft) if !ft.noexcept));
                    let mut st = Std::identity(p.clone());
                    st.qual = dropped;
                    Ok(Some(Ics::std(st)))
                }
                _ => Err(()),
            };
        }
        if let EK::TemplId(None, fs, raw) = &a.k {
            // ICS-47, OVL-19: a template name with explicit arguments
            // names the specializations that match the parameter type.
            if matches!(&p.t, Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) if matches!(x.t, Ty::Func(_))) {
                let (fs, raw) = (fs.clone(), raw.clone());
                let target = p.unqual();
                let me = self.d.muted_errors;
                self.d.muted += 1;
                let v = self.target_specs(&fs, Some(&raw), &target, a.line);
                self.d.muted -= 1;
                if self.d.muted_errors != me {
                    return Err(());
                }
                return match v.map(|v| v.len()) {
                    Some(0) => Ok(None),
                    Some(1) => Ok(Some(Ics::std(Std::identity(p.clone())))),
                    _ => Err(()),
                };
            }
        }
        if a.ty.is_unknown() || matches!(a.k, EK::Overloads(_) | EK::BoundMember(..)) {
            return Err(());
        }
        match &p.t {
            Ty::LRef(i) | Ty::RRef(i) => {
                let rref = matches!(p.t, Ty::RRef(_));
                let i = (**i).clone();
                self.ics_ref(a, &i, rref, std_only)
            }
            _ => self.ics_value(a, &p.unqual(), std_only),
        }
    }

    /// The front end knows every member of class `c`: it is complete,
    /// and no member declaration failed to parse.
    pub fn class_ok(&self, c: usize) -> bool {
        !self.class_uncertain(c) && self.classes[c].complete && !self.scopes[self.classes[c].scope].uncertain
    }

    pub fn ics_value_pub(&mut self, a: &Expr, p: &QTy) -> R {
        if self.in_template() {
            return Err(());
        }
        self.ics_value(a, p, false)
    }

    /// A parameter that is not a reference.
    fn ics_value(&mut self, a: &Expr, p: &QTy, std_only: bool) -> R {
        let at = nonref(&a.ty);
        if let Ty::Class(pc) = p.t {
            if let Ty::Class(ac) = at.t {
                if ac == pc {
                    // ICS-2: a class argument for the same class is exact.
                    return Ok(Some(Ics::std(Std::identity(p.clone()))));
                }
                if !self.class_ok(ac) {
                    return Err(());
                }
                if self.is_base_of(pc, ac) {
                    return Ok(Some(Ics::std(Std { rank: 2, sec: Sec::ClassBase(ac, pc), qual: false, to: p.clone(), refb: None })));
                }
            }
            if self.init_list_elem(pc).is_some() {
                return if matches!(at.t, Ty::Class(_)) { Err(()) } else { Ok(None) };
            }
            if !self.class_ok(pc) {
                return Err(());
            }
            if std_only {
                return Ok(None);
            }
            return self.ics_user_to_class(a, pc);
        }
        if let Ty::Class(ac) = at.t {
            if std_only {
                return Ok(None);
            }
            if !self.class_ok(ac) {
                return Err(());
            }
            return self.ics_user_from_class(a, ac, p, None);
        }
        self.ics_standard(a, p)
    }

    /// A standard sequence between types that are not classes.
    fn ics_standard(&mut self, a: &Expr, p: &QTy) -> R {
        let at = nonref(&a.ty);
        let null = self.is_null_ptr_constant(a);
        let mut from = at.t.clone();
        match &from {
            Ty::Array(e, _) => from = Ty::Ptr(e.clone()),
            Ty::Func(_) => from = Ty::Ptr(Box::new(at.unqual())),
            _ => {}
        }
        if from.contains_unknown() || matches!(from, Ty::ExtFloat) || matches!(p.t, Ty::ExtFloat) {
            return Err(());
        }
        let to = p.t.clone();
        if from == to {
            return Ok(Some(Ics::std(Std::identity(p.clone()))));
        }
        let nerr = self.d.list.len();
        self.d.muted += 1;
        let ok = self.implicit(a.clone(), p, false);
        self.d.muted -= 1;
        self.d.list.truncate(nerr);
        if ok.is_err() {
            return Ok(None);
        }
        let mk = |rank: u8, sec: Sec, qual: bool| Ok(Some(Ics::std(Std { rank, sec, qual, to: p.clone(), refb: None })));
        let arith = |t: &Ty| t.is_arithmetic() || matches!(t, Ty::Enum(_));
        match (&from, &to) {
            (Ty::Ptr(_) | Ty::MemPtr(..) | Ty::NullPtr, Ty::Bool) => mk(2, Sec::PtrBool, false),
            (f, t) if arith(f) && t.is_arithmetic() => {
                if let Some(bp) = self.bitfield_promotion(a) {
                    // ICS-2, CONV-1: a bit-field promotes by its width.
                    return if &bp == t { mk(1, Sec::Promo, false) } else { mk(2, Sec::Arith, false) };
                }
                if let Ty::Enum(e) = f {
                    if let Some(u) = self.enums[*e].fixed.clone() {
                        if !self.enums[*e].scoped && &u == t {
                            return mk(1, Sec::PromoUnderlying, false);
                        }
                    }
                }
                if (f.is_integral() || matches!(f, Ty::Enum(_))) && &self.promoted(f) == t {
                    mk(1, Sec::Promo, false)
                } else if *f == Ty::Float && *t == Ty::Double {
                    mk(1, Sec::Promo, false)
                } else {
                    mk(2, Sec::Arith, false)
                }
            }
            (_, Ty::Ptr(_) | Ty::MemPtr(..)) if null && !matches!(from, Ty::Ptr(_) | Ty::MemPtr(..)) => mk(2, Sec::Null, false),
            (Ty::NullPtr, _) => mk(2, Sec::Null, false),
            (Ty::Ptr(fp), Ty::Ptr(tp)) => {
                let added = (tp.c && !fp.c) || (tp.v && !fp.v);
                if tp.t == Ty::Void && fp.t != Ty::Void {
                    mk(2, Sec::PtrVoid(fp.t.clone()), added)
                } else if let (Ty::Class(d), Ty::Class(b)) = (&fp.t, &tp.t) {
                    if d != b {
                        mk(2, Sec::PtrBase(*d, *b), added)
                    } else {
                        mk(0, Sec::None, true)
                    }
                } else {
                    // a qualification adjustment, also through several
                    // levels, or noexcept dropped from a function pointer
                    mk(0, Sec::None, true)
                }
            }
            (Ty::MemPtr(fc, fa), Ty::MemPtr(tc, ta)) => {
                let added = (ta.c && !fa.c) || (ta.v && !fa.v);
                if fc != tc && !self.is_base_of(*fc, *tc) {
                    return Ok(None);
                }
                if fc != tc {
                    mk(2, Sec::MemBase(*fc, *tc), added)
                } else {
                    mk(0, Sec::None, true)
                }
            }
            _ => Err(()),
        }
    }

    /// ICS-11, ICS-12: a reference parameter to `inner`.
    fn ics_ref(&mut self, a: &Expr, inner: &QTy, rref: bool, std_only: bool) -> R {
        let at = nonref(&a.ty);
        let is_lv = a.cat == Cat::L;
        let refb = |to_rvalue: bool, fn_lvalue: bool| RefB { rref, to_rvalue, fn_lvalue, referenced: inner.clone(), obj_noref: false };
        if let Ty::Func(_) = inner.t {
            // ICS-12: both kinds of reference bind a function lvalue.
            if let (Ty::Func(af), Ty::Func(pf)) = (&at.t, &inner.t) {
                // A noexcept function binds a reference to the same type
                // without noexcept.
                let mut a2 = (**af).clone();
                a2.noexcept = pf.noexcept;
                if a2 != **pf || (pf.noexcept && !af.noexcept) {
                    return Ok(None);
                }
                let mut s = Std::identity(inner.clone());
                s.refb = Some(refb(false, true));
                return Ok(Some(Ics::std(s)));
            }
            return Err(());
        }
        let related_base = match (&at.t, &inner.t) {
            (Ty::Class(x), Ty::Class(y)) if x != y => {
                if !self.class_ok(*x) {
                    return Err(());
                }
                self.is_base_of(*y, *x).then_some((*x, *y))
            }
            _ => None,
        };
        // An array's qualifiers are those of its elements (CV-6).
        let related = at.t == inner.t || crate::templ::strip_cv(&at, true, true).t == crate::templ::strip_cv(inner, true, true).t || related_base.is_some();
        let cv_ok = (!at.c || inner.c) && (!at.v || inner.v);
        let const_lref = !rref && inner.c && !inner.v;
        if related && cv_ok {
            let binds = if rref { !is_lv } else { is_lv || const_lref };
            if binds {
                let (rank, sec) = match related_base {
                    Some((x, y)) => (2, Sec::ClassBase(x, y)),
                    None => (0, Sec::None),
                };
                let mut s = Std { rank, sec, qual: false, to: inner.clone(), refb: None };
                s.refb = Some(refb(!is_lv, false));
                return Ok(Some(Ics::std(s)));
            }
        }
        if let Ty::Class(ac) = at.t {
            if !related && !std_only {
                if !self.class_ok(ac) {
                    return Err(());
                }
                // ICS-11: a conversion function whose result binds.
                return self.ics_user_from_class(a, ac, inner, Some(rref));
            }
        }
        if related || !(rref || const_lref) {
            // ICS-12: no sequence.
            return Ok(None);
        }
        // A temporary of the referenced type (ICS-11).
        let r = self.ics_value(a, &inner.unqual(), std_only)?;
        Ok(r.map(|mut x| {
            x.s.refb = Some(refb(true, false));
            x
        }))
    }

    /// The conversion functions of class `c` and its bases. A function
    /// of a derived class hides one of a base with the same target type.
    pub fn conv_fns(&self, c: usize, depth: u32, out: &mut Vec<DeclId>) {
        if depth > 16 {
            return;
        }
        let sc = self.classes[c].scope;
        let mut names: Vec<(&String, &Vec<DeclId>)> = self.scopes[sc].names.iter().collect();
        names.sort_by_key(|x| x.0.clone());
        // A conversion function of a derived class hides the one of a base
        // with the same name; the overloads of one class all count (CVF-4).
        let before = out.len();
        for (_, ids) in names {
            for &d in ids {
                if self.decls[d].scope != sc {
                    continue;
                }
                if let Some(f) = self.func(d) {
                    if f.kind == FnKind::Conversion && !out[..before].iter().any(|&o| self.decls[o].name == self.decls[d].name) && !out.contains(&d) {
                        out.push(d);
                    }
                }
            }
        }
        for b in self.classes[c].bases.clone() {
            if let Some(bc) = b.class {
                self.conv_fns(bc, depth + 1, out);
            }
        }
    }

    /// The sequence from object `a` to the implicit object parameter of
    /// member function `f` (ICS-13). The class is the class of the object.
    pub fn ics_object(&self, a: &Expr, f: &FuncInfo) -> Option<Ics> {
        if f.is_static {
            return Some(Ics::slot());
        }
        let at = nonref(&a.ty);
        let is_lv = a.cat == Cat::L;
        if (at.c && !f.fty.cv_c) || (at.v && !f.fty.cv_v) {
            return None;
        }
        let referenced = QTy { t: at.t.clone(), c: f.fty.cv_c, v: f.fty.cv_v };
        let ok = match f.fty.refq {
            0 => true,
            1 => is_lv || (f.fty.cv_c && !f.fty.cv_v),
            _ => !is_lv,
        };
        if !ok {
            return None;
        }
        let mut s = Std::identity(referenced.clone());
        s.refb = Some(RefB { rref: f.fty.refq == 2, to_rvalue: !is_lv, fn_lvalue: false, referenced, obj_noref: f.fty.refq == 0 });
        Some(Ics::std(s))
    }

    /// An expression for the result of a call of `f`.
    fn result_expr(&self, f: &FuncInfo, line: u32) -> Expr {
        let (t, cat) = crate::expr2::ret_cat(&f.fty.ret);
        Expr::new(EK::Unknown("result".into()), t, cat, line)
    }

    /// ICS-3, ICS-4, ICS-5, ICS-37: a user-defined sequence from a class
    /// object through one of its conversion functions. `to_ref` is Some
    /// when `p` is the referenced type of a reference parameter, with
    /// whether the reference is an rvalue reference.
    fn ics_user_from_class(&mut self, a: &Expr, ac: usize, p: &QTy, to_ref: Option<bool>) -> R {
        let mut fns = Vec::new();
        self.conv_fns(ac, 0, &mut fns);
        let mut cands: Vec<UCand> = Vec::new();
        for fid in fns {
            let fid = if self.is_func_templ(fid) {
                // DED-31: the specialization for the needed type.
                match self.conv_spec(fid, p, to_ref) {
                    crate::templ::SpecRes::Ok(s) => s,
                    crate::templ::SpecRes::Fail => continue,
                    crate::templ::SpecRes::Unknown => return Err(()),
                }
            } else {
                fid
            };
            let f = self.func(fid).unwrap().clone();
            // ICS-8: a deleted function still forms a sequence. CVF-3: a
            // contextual conversion to bool also uses an explicit
            // conversion function to bool.
            if f.explicit && !self.direct_conv && !(self.contextual && f.fty.ret.t == Ty::Bool && p.t == Ty::Bool) {
                continue;
            }
            if f.fty.ret.is_unknown() {
                return Err(());
            }
            let Some(first) = self.ics_object(a, &f) else { continue };
            let r = self.result_expr(&f, a.line);
            let third = match to_ref {
                Some(rref) => self.ics_ref(&r, p, rref, true)?,
                None => self.ics_value(&r, p, true)?,
            };
            let Some(third) = third else { continue };
            if third.kind != IK::Std {
                continue;
            }
            if f.explicit && self.direct_conv && third.s.sec != Sec::None {
                // ICS-45: in a direct-initialization, an explicit conversion
                // function serves only with a qualification adjustment left.
                continue;
            }
            if (f.template || self.spec_of.contains_key(&fid)) && third.s.rank != 0 {
                // ICS-5: a template only with an exact-match third part.
                continue;
            }
            cands.push(UCand { f: fid, first, third: third.s });
        }
        // OVL-28, ORD-8: the order of the templates of the
        // specializations.
        self.tie_more.clear();
        let specs: Vec<DeclId> = cands.iter().map(|c| c.f).filter(|d| self.spec_of.contains_key(d)).collect();
        for &x in &specs {
            for &y in &specs {
                if x != y {
                    let (tx, ty) = (self.spec_of[&x].0, self.spec_of[&y].0);
                    if tx != ty {
                        let (a, b) = self.func_order(tx, ty, None);
                        if a && !b {
                            self.tie_more.push((x, y));
                        }
                    }
                }
            }
        }
        let r = self.pick_user(cands, true);
        self.tie_more.clear();
        Ok(r)
    }

    /// ICS-3, ICS-4: a user-defined sequence to class `pc` through a
    /// converting constructor, or a conversion function of the argument.
    fn ics_user_to_class(&mut self, a: &Expr, pc: usize) -> R {
        let mut cands: Vec<UCand> = Vec::new();
        let ctors = self.classes[pc].ctors.clone();
        let ctors = if ctors.iter().any(|&cid| self.func(cid).is_some_and(|f| f.template)) {
            // DED-1: a converting constructor template.
            match self.expand_fn_templates(&ctors, std::slice::from_ref(a), a.line) {
                Some(v) => v,
                None => return Err(()),
            }
        } else {
            ctors
        };
        for cid in ctors {
            let Some(f) = self.func(cid).cloned() else { return Err(()) };
            if f.template {
                return Err(());
            }
            if f.explicit || f.params.is_empty() || f.params.len() - 1 > f.defaults {
                continue;
            }
            let pt = self.var(f.params[0]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
            // ICS-10: the first parameter takes only a standard sequence.
            let Some(first) = self.ics_form(a, &pt, true)? else { continue };
            cands.push(UCand { f: cid, first, third: Std::identity(QTy::new(Ty::Class(pc))) });
        }
        if let Ty::Class(ac) = nonref(&a.ty).t {
            let mut fns = Vec::new();
            self.conv_fns(ac, 0, &mut fns);
            for fid in fns {
                let f = self.func(fid).unwrap().clone();
                if f.explicit {
                    continue;
                }
                let rt = nonref(&f.fty.ret);
                let third = match rt.t {
                    Ty::Class(rc) if rc == pc => Std::identity(QTy::new(Ty::Class(pc))),
                    Ty::Class(rc) if self.is_base_of(pc, rc) => Std { rank: 2, sec: Sec::ClassBase(rc, pc), qual: false, to: QTy::new(Ty::Class(pc)), refb: None },
                    Ty::Unknown | Ty::Dep(_) => return Err(()),
                    _ => continue,
                };
                let Some(first) = self.ics_object(a, &f) else { continue };
                cands.push(UCand { f: fid, first, third });
            }
        }
        Ok(self.pick_user(cands, false))
    }

    /// The best candidate of a user-defined conversion: by the first
    /// sequences, then (for conversion functions) by the third parts
    /// with the cases of ICS-37. No best candidate gives the ambiguous
    /// sequence (ICS-7).
    fn pick_user(&self, cands: Vec<UCand>, by_third: bool) -> Option<Ics> {
        if cands.is_empty() {
            return None;
        }
        let conv = |f: DeclId| self.func(f).is_some_and(|f| f.kind == FnKind::Conversion);
        let better = |x: &UCand, y: &UCand| -> bool {
            let c = self.ics_cmp(&x.first, &y.first);
            if c != 0 {
                return c > 0;
            }
            // ICS-37, OVL-26: between two conversion functions, the better
            // third part.
            let t = if by_third || (conv(x.f) && conv(y.f)) { self.std_cmp(&x.third, &y.third, true) } else { 0 };
            if t != 0 {
                return t > 0;
            }
            // OVL-27, OVL-28.
            (!self.spec_of.contains_key(&x.f) && self.spec_of.contains_key(&y.f)) || self.tie_more.contains(&(x.f, y.f))
        };
        for (i, x) in cands.iter().enumerate() {
            if cands.iter().enumerate().all(|(j, y)| i == j || better(x, y)) {
                return Some(Ics { kind: IK::User, s: x.third.clone(), user: Some(UKey::Fn(x.f)), list: LT::No });
            }
        }
        Some(Ics::ambig())
    }

    /// ICS-15 to ICS-20: a braced list.
    fn ics_list(&mut self, a: &Expr, p: &QTy, std_only: bool) -> R {
        let EK::InitList(items) = &a.k else { return Err(()) };
        let items = items.clone();
        let designated = items.iter().any(|x| matches!(&x.k, EK::Unknown(s) if s.starts_with("designator")));
        let worst = |p: &P, v: &[Ics]| -> Ics {
            let mut w = v[0].clone();
            for x in &v[1..] {
                if p.ics_cmp(x, &w) < 0 {
                    w = x.clone();
                }
            }
            w
        };
        // References.
        if let Ty::LRef(i) | Ty::RRef(i) = &p.t {
            let rref = matches!(p.t, Ty::RRef(_));
            let inner = (**i).clone();
            if let Some(x) = self.list_binds_directly(&items, &inner) {
                // ICS-20, REF-6: the reference binds to the one element.
                if x.ty.is_unknown() {
                    return Err(());
                }
                if let (Ty::Array(_, None), Ty::Array(..)) = (&inner.t, &nonref(&x.ty).t) {
                    let t2 = QTy { t: nonref(&x.ty).t, c: inner.c, v: inner.v };
                    return self.ics_ref(&x, &t2, rref, std_only);
                }
                return self.ics_ref(&x, &inner, rref, std_only);
            }
            if !(rref || (inner.c && !inner.v)) {
                return Ok(None);
            }
            let r = if let Ty::Array(el, n) = &inner.t {
                // ICS-17: a reference to an array.
                if designated {
                    return Err(());
                }
                let el = (**el).clone();
                if let Some(n) = n {
                    if items.len() as u64 > *n {
                        return Ok(None);
                    }
                }
                let count = items.len() as u64;
                if let (1, EK::Str(u)) = (items.len(), &items[0].k) {
                    // ICS-38: one string literal for an array of characters,
                    // if the literal can initialize the array (LIST-6,
                    // LIST-15).
                    let st = match &items[0].ty.t {
                        Ty::Array(c, _) => c.t.clone(),
                        _ => return Err(()),
                    };
                    let compatible = st == el.t || (st == Ty::Char8 && matches!(el.t, Ty::Char | Ty::UChar)) || (st == Ty::Char && matches!(el.t, Ty::SChar | Ty::UChar));
                    let len = u.len() as u64 + 1;
                    if !compatible || n.is_some_and(|n| len > n) {
                        return Ok(None);
                    }
                    let mut s = Std::identity(inner.clone());
                    s.refb = None;
                    Some(Ics { kind: IK::Std, s, user: None, list: LT::Array(el.t.clone(), *n, len) })
                } else {
                    let mut seqs = Vec::new();
                    for x in &items {
                        match self.ics_form(x, &el, false)? {
                            Some(s) => seqs.push(s),
                            None => return Ok(None),
                        }
                    }
                    let mut w = if seqs.is_empty() { Ics::std(Std::identity(el.clone())) } else { worst(self, &seqs) };
                    w.list = LT::Array(el.t.clone(), *n, count);
                    Some(w)
                }
            } else {
                // ICS-20: a temporary of the referenced type.
                self.ics_list(a, &inner.unqual(), std_only)?
            };
            return Ok(r.map(|mut x| {
                x.s.refb = Some(RefB { rref, to_rvalue: true, fn_lvalue: false, referenced: inner.clone(), obj_noref: false });
                x
            }));
        }
        if let Ty::Class(pc) = p.t {
            if let Some(el) = self.init_list_elem(pc) {
                // ICS-16: each element converts to E. A designated list
                // has no elements of E (outside knowledge).
                if designated {
                    return Ok(None);
                }
                if el.is_unknown() {
                    return Err(());
                }
                let mut seqs = Vec::new();
                for x in &items {
                    match self.ics_form(x, &el, false)? {
                        Some(s) => seqs.push(s),
                        None => return Ok(None),
                    }
                }
                let mut w = if seqs.is_empty() { Ics::std(Std::identity(p.clone())) } else { worst(self, &seqs) };
                w.list = LT::InitList;
                return Ok(Some(w));
            }
            if !self.class_ok(pc) {
                return Err(());
            }
            let one_of_class = |p: &P, x: &Expr| match nonref(&x.ty).t {
                Ty::Class(xc) => xc == pc || p.is_base_of(pc, xc),
                _ => false,
            };
            if self.classes[pc].ctors.is_empty() {
                // ICS-18: an aggregate.
                if self.classes[pc].has_virtual || self.classes[pc].key == 2 {
                    return Err(());
                }
                if items.len() == 1 && !designated && one_of_class(self, &items[0]) {
                    return self.ics_value(&items[0], p, std_only);
                }
                if std_only {
                    return Ok(None);
                }
                let user = Ics { kind: IK::User, s: Std::identity(p.clone()), user: Some(UKey::Agg(pc)), list: LT::No };
                let fields = self.classes[pc].fields.clone();
                if designated {
                    // LIST-5: the named members, in any order.
                    for x in &items {
                        if let EK::Unknown(s) = &x.k {
                            let name = s.split_whitespace().nth(1).unwrap_or("");
                            if !fields.iter().any(|&f| self.decls[f].name == name) {
                                return Ok(None);
                            }
                        }
                    }
                    return Ok(Some(user));
                }
                if !self.classes[pc].bases.is_empty() {
                    return Err(());
                }
                if items.len() > fields.len() {
                    return Ok(None);
                }
                for (i, x) in items.iter().enumerate() {
                    let ft = match &self.decls[fields[i]].k {
                        DK::Field(fi) => fi.ty.clone(),
                        _ => return Err(()),
                    };
                    if matches!(ft.t, Ty::Array(..) | Ty::Class(_)) && !matches!(x.k, EK::InitList(_)) {
                        // brace elision
                        return Err(());
                    }
                    if self.ics_form(x, &ft, false)?.is_none() {
                        return Ok(None);
                    }
                }
                return Ok(Some(user));
            }
            // ICS-19: a class with constructors.
            if designated {
                return Ok(None);
            }
            let ctors = self.classes[pc].ctors.clone();
            if ctors.iter().any(|&c| self.func(c).is_none_or(|f| f.template)) {
                return Err(());
            }
            if std_only {
                return Ok(None);
            }
            // Phase 1: the initializer-list constructors (ILC-2).
            let mut cands: Vec<(DeclId, Vec<Ics>)> = Vec::new();
            for &cid in &ctors {
                let f = self.func(cid).unwrap().clone();
                if f.params.is_empty() || f.params.len() - 1 > f.defaults {
                    continue;
                }
                let pt = nonref(&self.var(f.params[0]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()));
                let Ty::Class(lc) = pt.t else { continue };
                let Some(el) = self.init_list_elem(lc) else { continue };
                let mut seqs = Vec::new();
                let mut ok = true;
                for x in &items {
                    match self.ics_form(x, &el, false)? {
                        Some(s) => seqs.push(s),
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok && !(items.is_empty() && self.default_ctor(pc).is_some()) {
                    cands.push((cid, vec![if seqs.is_empty() { Ics::std(Std::identity(el.clone())) } else { worst(self, &seqs) }]));
                }
            }
            if cands.is_empty() {
                if items.len() == 1 && one_of_class(self, &items[0]) {
                    return self.ics_value(&items[0], p, std_only);
                }
                // Phase 2: all constructors, with the elements as arguments.
                for &cid in &ctors {
                    let f = self.func(cid).unwrap().clone();
                    let np = f.params.len();
                    if items.len() > np || items.len() + f.defaults < np {
                        continue;
                    }
                    let mut seqs = Vec::new();
                    let mut ok = true;
                    for (i, x) in items.iter().enumerate() {
                        let pt = self.var(f.params[i]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                        // ICS-10: an inner list for a parameter of the class.
                        let own = matches!(nonref(&pt).t, Ty::Class(c) if c == pc) && items.len() == 1 && matches!(x.k, EK::InitList(_));
                        match self.ics_form(x, &pt, own)? {
                            Some(s) => seqs.push(s),
                            None => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        cands.push((cid, seqs));
                    }
                }
            }
            if cands.is_empty() {
                return Ok(None);
            }
            let best = self.best_of(&cands);
            let user = match best {
                Some(i) => Ics { kind: IK::User, s: Std::identity(p.clone()), user: Some(UKey::Fn(cands[i].0)), list: LT::No },
                // ICS-19: the ambiguous sequence, marked with the class so
                // that ICS-8 can name CTOR-25.
                None => Ics { user: Some(UKey::Agg(pc)), ..Ics::ambig() },
            };
            return Ok(Some(user));
        }
        if matches!(p.t, Ty::Array(..)) {
            return Err(());
        }
        // ICS-15: a parameter that is not a class or a reference.
        if designated {
            return Ok(None);
        }
        match items.len() {
            0 => Ok(Some(Ics::std(Std::identity(p.clone())))),
            1 if !matches!(items[0].k, EK::InitList(_)) => self.ics_form(&items[0], p, std_only),
            _ => Ok(None),
        }
    }

    /// OVL-25, OVL-34: the index of the viable function that is better
    /// than each other one, by the sequences of its arguments. The
    /// tie-breakers of OVL-26 to OVL-33 are left to the callers, which
    /// give up on the candidates that need them.
    pub fn best_of(&self, cands: &[(DeclId, Vec<Ics>)]) -> Option<usize> {
        let better = |fx: DeclId, x: &Vec<Ics>, fy: DeclId, y: &Vec<Ics>| {
            let mut some = false;
            for (a, b) in x.iter().zip(y.iter()) {
                let c = self.ics_cmp(a, b);
                if c < 0 {
                    return false;
                }
                if c > 0 {
                    some = true;
                }
            }
            if !some {
                // OVL-27: a plain function beats a template specialization.
                if !self.spec_of.contains_key(&fx) && self.spec_of.contains_key(&fy) && self.func(fx).is_some() {
                    return true;
                }
                // OVL-28: the more specialized template wins.
                if self.tie_more.contains(&(fx, fy)) {
                    return true;
                }
            }
            some
        };
        (0..cands.len()).find(|&i| (0..cands.len()).all(|j| i == j || better(cands[i].0, &cands[i].1, cands[j].0, &cands[j].1)))
    }

    /// ICS-21 to ICS-36: 1 when `a` is better, -1 when `b` is better, 0
    /// when neither is.
    pub fn ics_cmp(&self, a: &Ics, b: &Ics) -> i8 {
        if a.kind == IK::Slot || b.kind == IK::Slot {
            // OVL-25: against the user-defined first sequence of a
            // surrogate candidate, the object slot of a static member is
            // better; against a standard sequence it takes no part.
            if a.kind == IK::Slot && matches!(b.kind, IK::User | IK::Ambig) {
                return 1;
            }
            if b.kind == IK::Slot && matches!(a.kind, IK::User | IK::Ambig) {
                return -1;
            }
            return 0;
        }
        // Step 1: list targets (ICS-22 to ICS-24).
        match (&a.list, &b.list) {
            (LT::InitList, x) if *x != LT::InitList => return 1,
            (x, LT::InitList) if *x != LT::InitList => return -1,
            (LT::Array(e1, n1, c1), LT::Array(e2, n2, c2)) if e1 == e2 => {
                let s1 = n1.unwrap_or(*c1);
                let s2 = n2.unwrap_or(*c2);
                if s1 != s2 {
                    return if s1 < s2 { 1 } else { -1 };
                }
                match (n1, n2) {
                    (Some(_), None) => return 1,
                    (None, Some(_)) => return -1,
                    _ => {}
                }
            }
            _ => {}
        }
        // Step 2: kind (ICS-21).
        let k = |i: &Ics| match i.kind {
            IK::Std | IK::Slot => 0,
            IK::User | IK::Ambig => 1,
            IK::Ellipsis => 2,
        };
        let (ka, kb) = (k(a), k(b));
        if ka != kb {
            return if ka < kb { 1 } else { -1 };
        }
        match a.kind {
            IK::Ellipsis => 0,
            IK::User | IK::Ambig => {
                // Step 3 (ICS-25).
                if a.kind == IK::Ambig || b.kind == IK::Ambig || a.user != b.user {
                    return 0;
                }
                self.std_cmp(&a.s, &b.s, false)
            }
            _ => self.std_cmp(&a.s, &b.s, false),
        }
    }

    /// Step 4 of ICS-P5 for two standard sequences. `third` adds the
    /// cases of ICS-37.
    pub fn std_cmp(&self, a: &Std, b: &Std, third: bool) -> i8 {
        // a. ICS-26: proper subsequence, without lvalue transformations.
        let sub = |x: &Std, y: &Std| (x.is_identity() && !y.is_identity()) || (x.sec != Sec::None && x.sec == y.sec && !x.qual && y.qual);
        if sub(a, b) {
            return 1;
        }
        if sub(b, a) {
            return -1;
        }
        // b. ICS-27: rank.
        if a.rank != b.rank {
            return if a.rank < b.rank { 1 } else { -1 };
        }
        // c. The one same-rank tie-breaker that applies (ICS-32 to
        // ICS-36, and ICS-42 to ICS-44 for the third parts of ICS-37).
        let one = |x: &Std, y: &Std| -> bool {
            // ICS-32: not to bool.
            if y.sec == Sec::PtrBool && x.sec != Sec::PtrBool {
                return true;
            }
            match (&x.sec, &y.sec) {
                // ICS-33: to a base pointer, not to void*.
                (Sec::PtrBase(d, _), Sec::PtrVoid(Ty::Class(d2))) if d == d2 => true,
                // ICS-34: to the underlying type.
                (Sec::PromoUnderlying, Sec::Promo) => true,
                // ICS-36: the nearer base.
                (Sec::PtrBase(d1, m), Sec::PtrBase(d2, b)) | (Sec::ClassBase(d1, m), Sec::ClassBase(d2, b)) if d1 == d2 && m != b => self.is_base_of(*b, *m),
                (Sec::MemBase(b1, m), Sec::MemBase(b2, d)) if b1 == b2 && m != d => self.is_base_of(*m, *d),
                // ICS-42: the nearer source, for the third parts.
                (Sec::PtrBase(m, b1), Sec::PtrBase(d, b2)) | (Sec::ClassBase(m, b1), Sec::ClassBase(d, b2)) if third && b1 == b2 && m != d => self.is_base_of(*m, *d),
                (Sec::MemBase(m, d1), Sec::MemBase(b, d2)) if third && d1 == d2 && m != b => self.is_base_of(*b, *m),
                // ICS-43: to void*, from the base.
                (Sec::PtrVoid(Ty::Class(b)), Sec::PtrVoid(Ty::Class(d))) if third && b != d => self.is_base_of(*b, *d),
                _ => false,
            }
        };
        if one(a, b) {
            return 1;
        }
        if one(b, a) {
            return -1;
        }
        if let (Some(ra), Some(rb)) = (&a.refb, &b.refb) {
            // d. ICS-28: an rvalue reference to an rvalue (not for the
            // object parameter of ICS-40).
            if ra.to_rvalue && rb.to_rvalue && !ra.obj_noref && !rb.obj_noref && ra.rref != rb.rref {
                return if ra.rref { 1 } else { -1 };
            }
            // e. ICS-29: an lvalue reference to a function.
            if ra.fn_lvalue && rb.fn_lvalue && ra.rref != rb.rref {
                return if ra.rref { -1 } else { 1 };
            }
        }
        // f. ICS-31: fewer cv-qualifiers added.
        if a.qual && b.qual && a.sec == b.sec && a.to != b.to {
            if self.qual_adds(&a.to, &b.to) {
                return 1;
            }
            if self.qual_adds(&b.to, &a.to) {
                return -1;
            }
        }
        // g. ICS-30: fewer cv-qualifiers on the referenced type.
        if let (Some(ra), Some(rb)) = (&a.refb, &b.refb) {
            if ra.referenced.t == rb.referenced.t {
                let (x, y) = (&ra.referenced, &rb.referenced);
                let x_sub = (!x.c || y.c) && (!x.v || y.v);
                let y_sub = (!y.c || x.c) && (!y.v || x.v);
                if x_sub && !y_sub {
                    return 1;
                }
                if y_sub && !x_sub {
                    return -1;
                }
            }
        }
        0
    }

    /// Does `x` convert to `y` by adding cv-qualifiers (ICS-31)?
    fn qual_adds(&self, x: &QTy, y: &QTy) -> bool {
        match (&x.t, &y.t) {
            (Ty::Ptr(a), Ty::Ptr(b)) => {
                if (a.c && !b.c) || (a.v && !b.v) {
                    return false;
                }
                if a.t == b.t {
                    return true;
                }
                self.qual_adds(a, b)
            }
            (a, b) => a == b && (!x.c || y.c) && (!x.v || y.v),
        }
    }

    /// Overload resolution for a call (OVL-P1): the viable functions
    /// (OVL-22, OVL-23), then the best one (OVL-25, OVL-34).
    /// Ok(Some(i)): candidate i. Ok(None): no viable function is better
    /// than each of the others (OVL-36). Err: the front end cannot form
    /// every sequence, or a candidate needs a step that it does not
    /// model; `ics_no_viable` is set when every sequence was formed and
    /// no candidate is viable (OVL-35).
    #[allow(clippy::type_complexity)]
    pub fn ics_select(&mut self, obj: Option<&Expr>, cands: &[DeclId], args: &[Expr]) -> Result<Option<(DeclId, Vec<Ics>)>, ()> {
        let cands = self.same_entities_removed(cands);
        let mut viable: Vec<(DeclId, Vec<Ics>)> = Vec::new();
        for &c in &cands {
            let Some(f) = self.func(c).cloned() else { return Err(()) };
            if f.template || f.opaque || f.constrained {
                // Deduction is not modeled here.
                return Err(());
            }
            // OVL-24: a function whose constraints are not satisfied is not
            // viable.
            match self.fn_sat(c) {
                crate::cons::Sat::Yes => {}
                crate::cons::Sat::Unknown => return Err(()),
                _ => continue,
            }
            let mut all: Vec<Expr> = Vec::new();
            let mut seqs = Vec::new();
            let member = f.class.is_some() && f.kind != FnKind::Ctor;
            match obj {
                Some(o) if member && f.explicit_obj => all.push(o.clone()),
                Some(o) if member => match self.ics_object(o, &f) {
                    Some(s) => seqs.push(s),
                    None => continue,
                },
                None if member && f.explicit_obj => return Err(()),
                // OVL-3: a call with no object still compares the members.
                // Their object slots take no part (outside knowledge).
                None if member => seqs.push(Ics::slot()),
                _ => {}
            }
            all.extend(args.iter().cloned());
            let mut ptys = f.fty.params.clone();
            if f.explicit_obj && f.params.len() == ptys.len() + 1 {
                // The function type leaves out the explicit object parameter.
                ptys.insert(0, self.var(f.params[0]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()));
            }
            let np = ptys.len();
            // OVL-22: the parameter count fits.
            if all.len() + f.defaults.min(np) < np || (all.len() > np && !f.fty.variadic) {
                continue;
            }
            // OVL-23: a sequence for each argument.
            let mut ok = true;
            for (i, a) in all.iter().enumerate() {
                if i >= np {
                    if is_list(a) {
                        // A braced list matches no `...` (outside the
                        // specification; the classes grid of the conversion
                        // sequences page shows it).
                        ok = false;
                        break;
                    }
                    seqs.push(Ics::ellipsis());
                    continue;
                }
                match self.ics_form(a, &ptys[i], false)? {
                    Some(s) => seqs.push(s),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                viable.push((c, seqs));
            }
        }
        if viable.is_empty() {
            // Every sequence was formed and no candidate is viable.
            self.ics_no_viable = true;
            return Err(());
        }
        let n = viable.iter().map(|v| v.1.len()).max().unwrap();
        if viable.iter().any(|v| v.1.len() != n) {
            return Err(());
        }
        // OVL-28: the order of the templates of the specializations.
        self.tie_more.clear();
        let specs: Vec<DeclId> = viable.iter().map(|v| v.0).filter(|d| self.spec_of.contains_key(d)).collect();
        if specs.len() > 1 {
            for &a in &specs {
                for &b in &specs {
                    if a != b && self.spec_more_specialized(a, b, args.len()) {
                        self.tie_more.push((a, b));
                    }
                }
            }
        }
        // OVL-30: of two functions that are not templates, with the same
        // parameters, the more constrained one.
        let plain: Vec<DeclId> = viable.iter().map(|v| v.0).filter(|d| !self.spec_of.contains_key(d)).collect();
        if plain.iter().any(|d| self.fn_cons.contains_key(d)) {
            for &a in &plain {
                for &b in &plain {
                    if a == b {
                        continue;
                    }
                    let (fa, fb) = (self.func(a).cloned().unwrap(), self.func(b).cloned().unwrap());
                    if fa.fty.params != fb.fty.params || fa.fty.cv_c != fb.fty.cv_c || fa.fty.refq != fb.fty.refq {
                        continue;
                    }
                    let (ca, cb) = (self.fn_cons.get(&a).cloned(), self.fn_cons.get(&b).cloned());
                    if self.more_constrained(&ca, u32::MAX, &[], &cb, u32::MAX, &[]) {
                        self.tie_more.push((a, b));
                    }
                }
            }
        }
        // CTAD-19 to CTAD-22 (ctad.rs).
        let extra = self.extra_ties.clone();
        self.tie_more.extend(extra);
        let r = self.best_of(&viable).map(|i| viable[i].clone());
        self.tie_more.clear();
        Ok(r)
    }

    /// One candidate for each function. Two declarations of a function
    /// with C language linkage in different namespaces name one function
    /// (LSP-4). A member of a derived class hides a member of a base
    /// with the same parameters that a using-declaration brings in
    /// (DER-3).
    pub fn same_entities_removed(&self, cands: &[DeclId]) -> Vec<DeclId> {
        let same_params = |a: &FuncInfo, b: &FuncInfo| a.fty.params == b.fty.params && a.fty.variadic == b.fty.variadic && a.fty.cv_c == b.fty.cv_c && a.fty.cv_v == b.fty.cv_v && a.fty.refq == b.fty.refq;
        let mut out: Vec<DeclId> = Vec::new();
        'next: for &c in cands {
            let Some(f) = self.func(c) else {
                out.push(c);
                continue;
            };
            for (k, &o) in out.clone().iter().enumerate() {
                let Some(g) = self.func(o) else { continue };
                if !same_params(f, g) || f.template || g.template {
                    continue;
                }
                if self.c_fns.contains(&c) && self.c_fns.contains(&o) {
                    continue 'next;
                }
                if let (Some(fc), Some(gc)) = (f.class, g.class) {
                    if fc != gc && self.is_base_of(fc, gc) {
                        continue 'next;
                    }
                    if fc != gc && self.is_base_of(gc, fc) {
                        out[k] = c;
                        continue 'next;
                    }
                }
            }
            out.push(c);
        }
        out
    }

    /// An operator expression with a class operand: the operator
    /// function chosen by the sequences of its operands (ICS-13, ICS-28).
    /// Only when no built-in operator can compete: no class operand has a
    /// conversion function, except for `=`, whose built-in candidates
    /// take only a standard sequence on the left (ICS-10). None when the
    /// front end cannot decide.
    pub fn operator_select(&mut self, op: &str, l: &Expr, r: Option<&Expr>, line: u32) -> Option<Expr> {
        if self.in_template() {
            return None;
        }
        let name = format!("operator{}", op);
        let mut ops: Vec<&Expr> = vec![l];
        if let Some(r) = r {
            ops.push(r);
        }
        for o in &ops {
            if o.ty.is_unknown() || (matches!(o.k, EK::InitList(_)) && o.ty.is_unknown()) {
                return None;
            }
            if let Ty::Class(c) = nonref(&o.ty).t {
                if !self.class_ok(c) || self.init_list_elem(c).is_some() {
                    return None;
                }
                let assign_left = op == "=" && std::ptr::eq(*o, l);
                if !assign_left && self.class_converts(c) != Some(false) {
                    return None;
                }
            }
        }
        let lc = match nonref(&l.ty).t {
            Ty::Class(c) => Some(c),
            _ => None,
        };
        if op == "=" {
            // The implicit copy and move assignment operators are not
            // declared in the class scope; give up when they could apply.
            let Some(c) = lc else { return None };
            let rc = r.map(|r| nonref(&r.ty).t);
            // A class that declares a copy assignment operator has no
            // implicit copy or move assignment (CPY-4, CPY-5), so the
            // declared operators are all the candidates.
            let declares_copy = self.copy_cands(c, true, false).is_some_and(|v| !v.is_empty());
            if !declares_copy && (matches!(rc, Some(Ty::Class(_))) || !self.classes[c].ctors.is_empty()) {
                return None;
            }
        }
        let mut members: Vec<DeclId> = Vec::new();
        if let Some(c) = lc {
            if let Some(ids) = self.lookup_member(c, &name) {
                members = ids;
            }
        }
        let mut others: Vec<DeclId> = Vec::new();
        if op != "=" {
            if let (Some(ids), false) = self.lookup_full(&name) {
                others.extend(ids.into_iter().filter(|&d| self.func(d).is_some_and(|f| f.class.is_none())));
            }
            let args: Vec<Expr> = ops.iter().map(|e| (*e).clone()).collect();
            let (adl, certain) = self.adl_lookup(&name, &args);
            if !certain {
                return None;
            }
            for d in adl {
                if !others.contains(&d) && self.func(d).is_some() {
                    others.push(d);
                }
            }
        }
        // DED-1: an operator function template adds its specialization.
        if members.iter().chain(others.iter()).any(|&d| self.func(d).is_some_and(|f| f.template)) {
            // A comparison also has rewritten candidates (OVL-13), which
            // this selection does not form.
            if matches!(op, "==" | "!=" | "<" | ">" | "<=" | ">=" | "<=>") {
                return None;
            }
            let rest: Vec<Expr> = r.iter().map(|x| (*x).clone()).collect();
            members = self.expand_fn_templates(&members, &rest, line)?;
            let all: Vec<Expr> = ops.iter().map(|e| (*e).clone()).collect();
            others = self.expand_fn_templates(&others, &all, line)?;
        }
        let mut cands: Vec<(DeclId, Vec<Ics>)> = Vec::new();
        for &fid in members.iter().chain(others.iter()) {
            let f = self.func(fid)?.clone();
            if f.template || f.opaque || f.defaulted {
                // A defaulted comparison has properties that the front end
                // does not model here.
                return None;
            }
            let member = members.contains(&fid);
            let mut seqs = Vec::new();
            let mut args: Vec<&Expr> = Vec::new();
            if member {
                if f.explicit_obj {
                    // ICS-41: an explicit object parameter is an ordinary
                    // parameter, which a user-defined sequence can reach.
                    let pt = f.params.first().and_then(|&p| self.var(p)).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                    match self.ics_form(l, &pt, false) {
                        Ok(Some(s)) => seqs.push(s),
                        Ok(None) => continue,
                        Err(()) => return None,
                    }
                } else {
                    match self.ics_object(l, &f) {
                        Some(s) => seqs.push(s),
                        None => continue,
                    }
                }
                args.extend(ops.iter().skip(1).copied());
            } else {
                args.extend(ops.iter().copied());
            }
            if args.len() != f.fty.params.len() {
                continue;
            }
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                match self.ics_form(a, &f.fty.params[i], false) {
                    Ok(Some(s)) => seqs.push(s),
                    Ok(None) => {
                        ok = false;
                        break;
                    }
                    Err(()) => return None,
                }
            }
            if ok {
                cands.push((fid, seqs));
            }
        }
        if cands.is_empty() {
            return None;
        }
        // OVL-28: the order of the templates of the specializations.
        self.tie_more.clear();
        let specs: Vec<DeclId> = cands.iter().map(|v| v.0).filter(|d| self.spec_of.contains_key(d)).collect();
        if specs.len() > 1 {
            let n = ops.len();
            for &a in &specs {
                for &b in &specs {
                    if a != b && self.spec_more_specialized(a, b, n) {
                        self.tie_more.push((a, b));
                    }
                }
            }
        }
        let best = self.best_of(&cands);
        self.tie_more.clear();
        let i = best?;
        let fid = cands[i].0;
        let saved = std::mem::take(&mut self.call_adl);
        let x = if members.contains(&fid) {
            self.call_overloads_pub(Some(l.clone()), vec![fid], ops.iter().skip(1).map(|e| (*e).clone()).collect(), line)
        } else {
            self.call_overloads_pub(None, vec![fid], ops.iter().map(|e| (*e).clone()).collect(), line)
        };
        self.call_adl = saved;
        Some(x)
    }

    /// CTOR-25: no constructor of class `c` accepts the arguments. False
    /// when the front end cannot tell, or when an implicit copy or move
    /// constructor could take the one argument.
    pub fn ctor_no_match(&mut self, c: usize, args: &[Expr]) -> bool {
        if self.in_template() || !self.class_ok(c) || self.classes[c].ctors.is_empty() || args.is_empty() {
            return false;
        }
        if args.len() == 1 && (matches!(nonref(&args[0].ty).t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)) || matches!(args[0].k, EK::InitList(_))) {
            return false;
        }
        for cid in self.classes[c].ctors.clone() {
            let Some(f) = self.func(cid).cloned() else { return false };
            if f.template || f.fty.variadic {
                return false;
            }
            let np = f.params.len();
            if let Some(&p0) = f.params.first() {
                // An initializer-list constructor can take a braced list
                // whole (ILC-2); the caller does not say which form it has.
                match nonref(&self.var(p0).map(|v| v.ty.clone()).unwrap_or(QTy::unknown())).t {
                    Ty::Class(lc) if self.init_list_elem(lc).is_some() => return false,
                    Ty::Unknown | Ty::Dep(_) => return false,
                    _ => {}
                }
            }
            if args.len() > np || args.len() + f.defaults < np {
                continue;
            }
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                let pt = self.var(f.params[i]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                match self.ics_form(a, &pt, false) {
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        ok = false;
                        break;
                    }
                    Err(()) => return false,
                }
            }
            if ok {
                return false;
            }
        }
        true
    }
}
