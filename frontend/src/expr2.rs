// Calls, casts, initialization, and placeholder deduction.

use crate::ast::*;
use crate::decl::{DOp, DeclSpec, Declarator};
use crate::decl2::InitForm;
use crate::expr::CastKind;
use crate::parser::*;
use crate::types::*;

impl P {
    // ---------------- calls ----------------

    pub fn build_call(&mut self, callee: Expr, args: Vec<Expr>, line: u32) -> Expr {
        match callee.k.clone() {
            EK::Func(f) => self.call_overloads(None, vec![f], args, line),
            EK::Overloads(fs) => self.call_overloads(None, fs, args, line),
            EK::BoundMember(obj, fs) => self.call_overloads(Some(*obj), fs, args, line),
            EK::Unknown(_) => {
                let _ = self.args_rvalues(args);
                Expr::unknown(line, "call of unknown callee")
            }
            _ => {
                let ct = nonref(&callee.ty);
                match &ct.t {
                    Ty::Class(_) => Expr::unknown(line, "call operator"),
                    Ty::Unknown => Expr::unknown(line, "call of unknown callee"),
                    Ty::Func(_) | Ty::Ptr(_) => {
                        let c = self.rvalue(callee);
                        let Ty::Ptr(p) = &c.ty.t else {
                            return Expr::unknown(line, "call");
                        };
                        let Ty::Func(ft) = &p.t else {
                            self.error(line, "PFX-7", "call of a value that is not a function");
                            return Expr::unknown(line, "bad call");
                        };
                        let ft = (**ft).clone();
                        let args = self.convert_args(&ft, args, line, None);
                        let (t, cat) = ret_cat(&ft.ret);
                        Expr::new(EK::Call(Box::new(c), args, None), t, cat, line)
                    }
                    _ => {
                        self.error(line, "PFX-7", format!("called object of type {} is not a function", show(&ct)));
                        Expr::unknown(line, "bad call")
                    }
                }
            }
        }
    }

    pub fn adl_candidates_pub(&self, name: &str, args: &[Expr]) -> Vec<DeclId> {
        self.adl_candidates(name, args)
    }

    fn args_rvalues(&mut self, args: Vec<Expr>) -> Vec<Expr> {
        args
    }

    /// Overload resolution is outside the spec. This picks the single
    /// viable candidate by arity and implicit convertibility, or the
    /// candidate with the best conversion ranks; anything harder gives
    /// an unknown result.
    /// NAM-5: functions in the namespaces of class and enumeration
    /// arguments join the candidates of an unqualified call. The front
    /// end does not track whether the call was qualified.
    fn adl_candidates(&self, name: &str, args: &[Expr]) -> Vec<DeclId> {
        let mut out = Vec::new();
        for a in args {
            let t = nonref(&a.ty).t;
            let scope = match t {
                Ty::Class(c) => Some(self.classes[c].scope),
                Ty::Enum(e) => Some(self.enums[e].scope),
                Ty::Ptr(ref p) => match p.t {
                    Ty::Class(c) => Some(self.classes[c].scope),
                    _ => None,
                },
                _ => None,
            };
            let Some(mut s) = scope else { continue };
            // the class's own scope holds hidden friends
            if let Some(v) = self.scopes[s].names.get(name) {
                for &d in v {
                    if self.func(d).is_some_and(|f| f.class.is_none()) && !out.contains(&d) {
                        out.push(d);
                    }
                }
            }
            while let Some(p) = self.scopes[s].parent {
                s = p;
                if self.scopes[s].kind == SK::Namespace {
                    break;
                }
            }
            if let Some(v) = self.scopes[s].names.get(name) {
                for &d in v {
                    if self.func(d).is_some() && !out.contains(&d) {
                        out.push(d);
                    }
                }
            }
        }
        out
    }

    fn call_overloads(&mut self, obj: Option<Expr>, cands: Vec<DeclId>, args: Vec<Expr>, line: u32) -> Expr {
        let mut cands = cands;
        if obj.is_none() {
            if let Some(&c0) = cands.first() {
                let name = self.decls[c0].name.clone();
                if self.func(c0).is_some_and(|f| f.class.is_none()) {
                    for d in self.adl_candidates(&name, &args) {
                        if !cands.contains(&d) {
                            cands.push(d);
                        }
                    }
                }
            }
        }
        let any_template = cands.iter().any(|&c| self.func(c).is_some_and(|f| f.template));
        let obj_const = obj.as_ref().is_some_and(|o| {
            let t = match &o.k {
                EK::This => match &o.ty.t {
                    Ty::Ptr(p) => p.c,
                    _ => false,
                },
                _ => nonref(&o.ty).c,
            };
            t
        });
        let obj = obj.map(|o| {
            if let EK::This = o.k {
                let t = match &o.ty.t {
                    Ty::Ptr(p) => (**p).clone(),
                    _ => QTy::unknown(),
                };
                Expr::new(EK::Unary(UOp::Deref, Box::new(o)), t, Cat::L, line)
            } else {
                o
            }
        });
        let mut viable: Vec<(DeclId, Vec<u8>)> = Vec::new();
        let mut arity_ok_any = false;
        let mut const_blocked = false;
        for &c in &cands {
            let Some(f) = self.func(c).cloned() else { continue };
            if f.template {
                continue;
            }
            let np = f.fty.params.len();
            let min = np - f.defaults.min(np);
            if args.len() < min || (args.len() > np && !f.fty.variadic) {
                continue;
            }
            arity_ok_any = true;
            if obj.is_some() && f.class.is_some() && !f.is_static && obj_const && !f.fty.cv_c && f.kind != FnKind::Ctor {
                const_blocked = true;
                continue;
            }
            let mut ranks = Vec::new();
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                if i >= np {
                    ranks.push(4);
                    continue;
                }
                match self.conv_rank(a, &f.fty.params[i]) {
                    Some(r) => ranks.push(r),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                viable.push((c, ranks));
            }
        }
        if viable.is_empty() {
            if any_template || self.in_template() || args.iter().any(|a| a.ty.is_unknown() || matches!(a.k, EK::InitList(_) | EK::Unknown(_))) {
                return Expr::unknown(line, "call needing template deduction or unknown arguments");
            }
            if const_blocked {
                // CLS-5: no call of a non-const member function on a const object.
                self.error(line, "CLS-25", "non-const member function called on a const object");
                return Expr::unknown(line, "bad call");
            }
            if !arity_ok_any {
                // PFX-5: one argument for each parameter without a default.
                self.error(line, "PFX-5", "wrong number of arguments in a function call");
            } else {
                self.error(line, "PFX-16", "no viable function: an argument cannot be converted to the parameter type");
            }
            return Expr::unknown(line, "bad call");
        }
        if any_template && viable.iter().all(|v| v.1.iter().any(|&r| r > 0)) {
            return Expr::unknown(line, "call that may select a template");
        }
        // best viable: no worse in every argument, better in one
        let mut best: Option<usize> = None;
        for i in 0..viable.len() {
            let better_than_all = (0..viable.len()).all(|j| {
                if i == j {
                    return true;
                }
                let a = &viable[i].1;
                let b = &viable[j].1;
                let no_worse = a.iter().zip(b.iter()).all(|(x, y)| x <= y);
                let better = a.iter().zip(b.iter()).any(|(x, y)| x < y);
                no_worse && (better || a == b && false)
            });
            if better_than_all {
                best = Some(i);
                break;
            }
        }
        let Some(bi) = best else {
            if viable.iter().any(|v| v.1.contains(&3)) {
                return Expr::unknown(line, "ambiguous call (needs full overload resolution)");
            }
            if viable.len() > 1 && viable.iter().all(|v| v.1 == viable[0].1) {
                // identical ranks: needs tie-breakers on cv/ref qualification
                return Expr::unknown(line, "ambiguous call");
            }
            return Expr::unknown(line, "ambiguous call");
        };
        let fid = viable[bi].0;
        let f = self.func(fid).unwrap().clone();
        if f.deleted {
            // DEL-1: a program must not refer to a deleted function.
            self.error(line, "DEL-1", format!("call of deleted function '{}'", self.decls[fid].name));
        }
        if f.is_main {
            // TR-3: a program must not call main.
            self.error(line, "TR-3", "call of 'main'");
        }
        let args = self.convert_args(&f.fty, args, line, Some(fid));
        let (t, cat) = ret_cat(&f.fty.ret);
        let callee = match obj {
            Some(o) if f.class.is_some() && !f.is_static => o,
            _ => Expr::new(EK::Func(fid), QTy::new(Ty::Func(Box::new(f.fty.clone()))), Cat::L, line),
        };
        if cat == Cat::PR && !self.in_template() && self.is_abstract(&t.t) {
            // VIRT-8: a call that returns an abstract class.
            self.error(line, "VIRT-8", "call of a function that returns an abstract class");
        }
        let call = Expr::new(EK::Call(Box::new(callee), args, Some(fid)), t, cat, line);
        let in_lambda = self.fns.iter().any(|f| f.is_lambda);
        if f.consteval && !self.in_immediate_context() && self.in_unevaluated == 0 && !self.in_template() && !in_lambda {
            // (A lambda that calls a consteval function with non-constant
            // arguments becomes consteval itself, CEV-1.)
            // CEV-1: each call of a consteval function is a constant expression.
            match self.const_check(&call) {
                Ok(()) => {}
                Err(crate::consteval::CeErr::NotConst(r, m)) => self.error_because(line, "CEV-1", r, format!("call of consteval function is not a constant expression ({}: {})", r, m)),
                Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(line, format!("consteval call: {}", m)),
            }
        }
        call
    }

    /// Conversion rank of an argument to a parameter: 0 exact, 1
    /// promotion, 2 conversion, 3 user-defined or unknown. None: not
    /// convertible.
    fn conv_rank(&mut self, a: &Expr, p: &QTy) -> Option<u8> {
        if a.ty.is_unknown() || p.is_unknown() || is_raw_list(a) || matches!(a.k, EK::Overloads(_)) {
            return Some(3);
        }
        let at = nonref(&a.ty);
        if let Ty::LRef(inner) | Ty::RRef(inner) = &p.t {
            let is_l = matches!(p.t, Ty::LRef(_));
            if at.t == inner.t {
                if is_l && !inner.c && a.cat != Cat::L {
                    return None;
                }
                if !is_l && a.cat == Cat::L {
                    return None;
                }
                if at.c && !inner.c {
                    return None;
                }
                return Some(0);
            }
            if let (Ty::Class(x), Ty::Class(y)) = (&at.t, &inner.t) {
                if self.is_base_of(*y, *x) {
                    return Some(2);
                }
            }
            if is_l && !inner.c {
                if matches!(at.t, Ty::Class(_)) {
                    return Some(3);
                }
                return None;
            }
            return self.conv_rank(a, &inner.unqual()).map(|r| r.max(2));
        }
        if let Ty::Class(_) = p.t {
            if at.t == p.t {
                return Some(0);
            }
            return Some(3);
        }
        if let Ty::Class(c) = at.t {
            return if self.class_converts(c) == Some(false) { None } else { Some(3) };
        }
        let nerr = self.d.list.len();
        self.d.muted += 1;
        let r = self.implicit(a.clone(), &p.unqual(), false);
        self.d.muted -= 1;
        self.d.list.truncate(nerr);
        match r {
            Err(_) => None,
            Ok(_) => {
                let mut from = at.t.clone();
                if let Ty::Array(e, _) = &from {
                    from = Ty::Ptr(e.clone());
                }
                if from == p.t {
                    Some(0)
                } else if let (Ty::Ptr(x), Ty::Ptr(y)) = (&from, &p.t) {
                    if x.t == y.t {
                        Some(0) // qualification adjustment
                    } else {
                        Some(2)
                    }
                } else if self.promoted(&from) == p.t && from != p.t && from.is_integral() || (from == Ty::Float && p.t == Ty::Double) {
                    Some(1)
                } else {
                    Some(2)
                }
            }
        }
    }

    fn convert_args(&mut self, ft: &FnTy, args: Vec<Expr>, line: u32, fid: Option<DeclId>) -> Vec<Expr> {
        let np = ft.params.len();
        if fid.is_none() {
            if args.len() < np || (args.len() > np && !ft.variadic) {
                self.error(line, "PFX-5", "wrong number of arguments in a call through a pointer");
            }
        }
        let mut out = Vec::new();
        for (i, a) in args.into_iter().enumerate() {
            if i < np {
                let p = ft.params[i].clone();
                let al = a.line;
                let c = if is_raw_list(&a) {
                    self.convert_init(a, &p, InitForm::CopyList, al)
                } else {
                    match self.implicit(a, &p, false) {
                        Ok(x) => x,
                        Err(m) => {
                            self.error(al, "PFX-16", format!("argument {}: {}", i + 1, m));
                            Expr::unknown(al, "bad argument")
                        }
                    }
                };
                out.push(c);
            } else {
                // ellipsis: promotions
                let a = self.rvalue(a);
                out.push(a);
            }
        }
        out
    }

    // ---------------- casts ----------------

    pub fn build_cast(&mut self, kind: CastKind, ty: QTy, e: Expr, line: u32) -> Expr {
        if ty.is_unknown() {
            return Expr::unknown(line, "cast to unknown type");
        }
        let (target, cat) = match &ty.t {
            Ty::LRef(i) => ((**i).clone(), Cat::L),
            Ty::RRef(i) => ((**i).clone(), if matches!(i.t, Ty::Func(_)) { Cat::L } else { Cat::X }),
            _ => (ty.unqual(), Cat::PR),
        };
        if e.ty.is_unknown() || matches!(e.k, EK::Overloads(_) | EK::BoundMember(..)) {
            return Expr::new(EK::Unknown("cast of unknown operand".into()), target, cat, line);
        }
        if kind == CastKind::Dynamic {
            if !self.in_template() {
                self.check_dynamic_cast(&e, &ty, line);
            }
            return Expr::new(EK::Unknown("dynamic_cast".into()), target, cat, line);
        }
        if kind == CastKind::Reinterpret {
            if cat == Cat::PR {
                let et = nonref(&e.ty);
                if let (Ty::Ptr(_), t) = (&et.t, &target.t) {
                    if t.is_integral() && t.int_bits() < if ilp32() { 32 } else { 64 } {
                        self.error(line, "CAST-7", "reinterpret_cast of a pointer to a smaller integer");
                    }
                }
                if let (Ty::Ptr(a), Ty::Ptr(b)) = (&et.t, &target.t) {
                    if (a.c && !b.c) || (a.v && !b.v) {
                        self.error(line, "CAST-7", "reinterpret_cast removes a cv-qualifier");
                    }
                }
            }
            // CE-4 / CAST-8: not in a constant expression.
            return Expr::new(EK::Unknown("CE-4: reinterpret_cast".into()), target, cat, line);
        }
        if cat != Cat::PR {
            // cast to a reference: binds to the operand
            let et = nonref(&e.ty);
            if kind == CastKind::Static && cat == Cat::L && e.cat != Cat::L && !target.c {
                self.error(line, "CAST-2", "static_cast of an rvalue to a non-const lvalue reference");
            }
            if (kind == CastKind::Static) && ((et.c && !target.c) || (et.v && !target.v)) {
                self.error(line, "CAST-2", "static_cast removes a cv-qualifier");
            }
            if et.t == target.t || matches!((&et.t, &target.t), (Ty::Class(a), Ty::Class(b)) if self.is_base_of(*b, *a) || self.is_base_of(*a, *b)) {
                return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), target, cat, line);
            }
            return Expr::new(EK::Unknown("reference cast".into()), target, cat, line);
        }
        if target.t == Ty::Void {
            return self.conv(e, Conv::ToVoid, target);
        }
        if let Ty::Class(_) = target.t {
            return Expr::new(EK::Unknown("cast to class".into()), target, Cat::PR, line);
        }
        if let Ty::Class(c) = nonref(&e.ty).t {
            if self.class_converts(c) == Some(false) && kind != CastKind::Const {
                self.error(line, "CAST-1", "no conversion from a class without conversion functions");
            }
            return Expr::new(EK::Unknown("conversion function".into()), target, Cat::PR, line);
        }
        let null = self.is_null_ptr_constant(&e);
        let e = self.rvalue(e);
        let from = e.ty.t.clone();
        let to = target.t.clone();
        if kind == CastKind::Const {
            let ok = match (&from, &to) {
                (Ty::Ptr(a), Ty::Ptr(b)) => self.same_ignoring_cv(a, b) && !matches!(a.t, Ty::Func(_)),
                (Ty::MemPtr(..), Ty::MemPtr(..)) => true,
                _ => false,
            };
            if !ok {
                self.error(line, "CAST-4", "const_cast can only change cv-qualifiers of a pointer or reference");
                return Expr::unknown(line, "bad cast");
            }
            return Expr::new(EK::Conv(Conv::NoOp, Box::new(e)), target, Cat::PR, line);
        }
        let arith_like = |p: &P, t: &Ty| t.is_arithmetic() || matches!(t, Ty::Enum(_)) || p.is_unscoped_enum(t);
        if arith_like(self, &from) && arith_like(self, &to) {
            return Expr::new(EK::Conv(Conv::Arith, Box::new(e)), target, Cat::PR, line);
        }
        if to == Ty::Bool && matches!(from, Ty::Ptr(_) | Ty::MemPtr(..) | Ty::NullPtr) {
            return self.conv(e, Conv::ToBool, target);
        }
        match (&from, &to) {
            (_, Ty::Ptr(_)) | (_, Ty::MemPtr(..)) | (_, Ty::NullPtr) if null || from == Ty::NullPtr => {
                return self.conv(e, Conv::NullPtr, target);
            }
            (Ty::Ptr(a), Ty::Ptr(b)) => {
                let cv_ok = (!a.c || b.c) && (!a.v || b.v);
                let related = a.t == b.t
                    || a.t == Ty::Void
                    || b.t == Ty::Void
                    || matches!((&a.t, &b.t), (Ty::Class(x), Ty::Class(y)) if self.is_base_of(*x, *y) || self.is_base_of(*y, *x));
                let func_obj = matches!(a.t, Ty::Func(_)) != matches!(b.t, Ty::Func(_));
                if kind == CastKind::Static {
                    if !cv_ok {
                        self.error(line, "CAST-2", "static_cast removes a cv-qualifier");
                        return Expr::unknown(line, "bad cast");
                    }
                    if !related || func_obj {
                        self.error(line, "CAST-2", "static_cast between unrelated pointer types");
                        return Expr::unknown(line, "bad cast");
                    }
                }
                if a.t == Ty::Void && b.t != Ty::Void {
                    // CE-4: a pointer to void to a pointer to an object type.
                    return Expr::new(EK::Unknown("CE-4: conversion from void*".into()), target, Cat::PR, line);
                }
                if related && cv_ok && !func_obj {
                    return self.conv(e, Conv::PtrConv, target);
                }
                return Expr::new(EK::Unknown("CE-4: reinterpreting pointer cast".into()), target, Cat::PR, line);
            }
            (f, Ty::Ptr(_)) if f.is_integral() || matches!(f, Ty::Enum(_)) => {
                if kind == CastKind::Static {
                    self.error(line, "CAST-2", "static_cast of an integer to a pointer");
                    return Expr::unknown(line, "bad cast");
                }
                return Expr::new(EK::Unknown("CE-4: integer to pointer".into()), target, Cat::PR, line);
            }
            (Ty::Ptr(_), t) if t.is_integral() => {
                if kind == CastKind::Static {
                    self.error(line, "CAST-2", "static_cast of a pointer to an integer");
                    return Expr::unknown(line, "bad cast");
                }
                return Expr::new(EK::Unknown("CE-4: pointer to integer".into()), target, Cat::PR, line);
            }
            (Ty::MemPtr(..), Ty::MemPtr(..)) => return Expr::new(EK::Unknown("member pointer cast".into()), target, Cat::PR, line),
            _ => {}
        }
        match self.implicit(e, &target, true) {
            Ok(x) => x,
            Err(m) => {
                self.error(line, if kind == CastKind::Static { "CAST-2" } else { "CAST-9" }, format!("invalid cast: {}", m));
                Expr::unknown(line, "bad cast")
            }
        }
    }

    fn same_ignoring_cv(&self, a: &QTy, b: &QTy) -> bool {
        match (&a.t, &b.t) {
            (Ty::Ptr(x), Ty::Ptr(y)) => self.same_ignoring_cv(x, y),
            (x, y) => x == y,
        }
    }

    // ---------------- initialization ----------------

    /// Convert an initializer to the type of the declared object
    /// (INIT-4, LIST-1, LIST-2, LIST-6, REF rules).
    pub fn convert_init(&mut self, e: Expr, ty: &QTy, form: InitForm, line: u32) -> Expr {
        if ty.is_unknown() {
            return e;
        }
        let items: Option<Vec<Expr>> = match &e.k {
            EK::InitList(v) if e.ty.is_unknown() => Some(v.clone()),
            _ => None,
        };
        let is_list = matches!(form, InitForm::CopyList | InitForm::DirectList) || (form == InitForm::None && items.is_some());
        let is_paren = matches!(form, InitForm::Paren(_));
        // references
        if ty.t.is_ref() {
            let one = match (&items, is_list || is_paren) {
                (Some(v), true) => {
                    if v.len() != 1 {
                        if !(is_list && v.is_empty() && false) {
                            self.error(line, "REF-11", "reference initializer must hold exactly one expression");
                        }
                        return Expr::unknown(line, "bad reference init");
                    }
                    let x = v[0].clone();
                    if is_list {
                        if let Ty::LRef(i) | Ty::RRef(i) = &ty.t {
                            if self.is_narrowing(&x, &i.t) && !matches!(x.ty.t, Ty::Class(_)) && nonref(&x.ty).t != i.t {
                                self.error(line, "LIST-2", "narrowing conversion in a braced reference initializer");
                            }
                        }
                    }
                    x
                }
                _ => e,
            };
            return self.bind_ref(one, ty, line, "INIT-7");
        }
        match &ty.t {
            Ty::Array(el, n) => {
                let el = (**el).clone();
                // string literal for a character array (LIST-6)
                let str_src = match (&items, &e.k) {
                    (Some(v), _) if v.len() == 1 && matches!(v[0].k, EK::Str(_)) => Some(v[0].clone()),
                    (None, EK::Str(_)) => Some(e.clone()),
                    _ => None,
                };
                if let Some(s) = str_src {
                    if el.t.is_integral() && el.t != Ty::Bool && !matches!(el.t, Ty::Short | Ty::UShort | Ty::Int | Ty::UInt | Ty::Long | Ty::ULong | Ty::LLong | Ty::ULLong) {
                        let st = match &s.ty.t {
                            Ty::Array(c, _) => c.t.clone(),
                            _ => Ty::Unknown,
                        };
                        let compatible = st == el.t
                            || (st == Ty::Char8 && matches!(el.t, Ty::Char | Ty::UChar))
                            || (st == Ty::Char && matches!(el.t, Ty::SChar | Ty::UChar));
                        if !compatible {
                            // LIST-6: the literal's character type must match.
                            self.error(line, "LIST-15", "string literal of the wrong character type for the array");
                        }
                        if let (Some(n), EK::Str(u)) = (n, &s.k) {
                            if (u.len() as u64) + 1 > *n && (u.len() as u64) > *n {
                                self.error(line, "LIST-15", "string literal too long for the array");
                            } else if (u.len() as u64) + 1 > *n {
                                self.error(line, "LIST-15", "string literal and its null character do not fit in the array");
                            }
                        }
                        return Expr::new(EK::Conv(Conv::NoOp, Box::new(s)), ty.clone(), Cat::PR, line);
                    }
                }
                let Some(v) = items else {
                    if matches!(e.ty.t, Ty::Array(..)) {
                        // INIT-5: an array must not be initialized from an array.
                        self.error(line, "INIT-8", "array initialized from another array");
                    } else if !e.ty.is_unknown() {
                        self.error(line, "INIT-8", "array initialized from a value that is not a list");
                    }
                    return Expr::unknown(line, "bad array init");
                };
                if let Some(n) = n {
                    let nested_elision = matches!(el.t, Ty::Array(..) | Ty::Class(_)) && v.iter().any(|x| !matches!(x.k, EK::InitList(_)));
                    if !nested_elision && v.len() as u64 > *n {
                        // DCL-5: not more elements than the bound.
                        self.error(line, "DCL-19", "too many initializers for the array");
                        return Expr::unknown(line, "bad array init");
                    }
                    if nested_elision {
                        return Expr::new(EK::Unknown("brace elision".into()), ty.clone(), Cat::PR, line);
                    }
                }
                let sub_form = if is_paren { InitForm::Copy } else { InitForm::CopyList };
                let mut out = Vec::new();
                for x in v {
                    let xl = x.line;
                    let f = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else { sub_form };
                    out.push(self.convert_init(x, &el, f, xl));
                }
                Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line)
            }
            Ty::Class(c) => {
                let c = *c;
                if !self.classes[c].complete && !self.in_template() {
                    self.error(line, "TYN-7", "object of incomplete class type");
                    return Expr::unknown(line, "incomplete");
                }
                let simple_aggr = self.is_simple_aggregate(c);
                match (&items, simple_aggr) {
                    (Some(v), true) if is_list || is_paren => {
                        let fields = self.classes[c].fields.clone();
                        let is_union = self.classes[c].key == 2;
                        let nfields = if is_union { 1.min(fields.len()) } else { fields.len() };
                        if v.iter().any(|x| matches!(x.k, EK::Unknown(ref s) if s == "designator")) {
                            return Expr::new(EK::Unknown("designated initializers".into()), ty.clone(), Cat::PR, line);
                        }
                        if v.len() == 1 && nonref(&v[0].ty).t == Ty::Class(c) {
                            return v[0].clone();
                        }
                        let any_nested_aggr = fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if matches!(fi.ty.t, Ty::Array(..) | Ty::Class(_))));
                        if v.len() > nfields && !any_nested_aggr {
                            // LIST-3: not more values than members.
                            self.error(line, "LIST-12", "too many initializers for the aggregate");
                            return Expr::unknown(line, "bad aggregate init");
                        }
                        if any_nested_aggr {
                            return Expr::new(EK::Unknown("aggregate with nested aggregates".into()), ty.clone(), Cat::PR, line);
                        }
                        let mut out = Vec::new();
                        for (i, x) in v.iter().enumerate() {
                            let fty = match &self.decls[fields[i]].k {
                                DK::Field(fi) => fi.ty.clone(),
                                _ => QTy::unknown(),
                            };
                            let xl = x.line;
                            let f = if is_paren { InitForm::Copy } else { InitForm::CopyList };
                            let f = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else { f };
                            out.push(self.convert_init(x.clone(), &fty, f, xl));
                        }
                        // missing members: default member initializers or value-init
                        for &f in fields.iter().skip(v.len()).take(nfields.saturating_sub(v.len())) {
                            if let DK::Field(fi) = &self.decls[f].k {
                                if fi.ty.t.is_ref() && fi.init.is_none() {
                                    // LIST-3: a reference member needs a value.
                                    self.error(line, "LIST-12", "reference member without an initializer in the list");
                                }
                            }
                        }
                        Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line)
                    }
                    (None, _) => {
                        let et = nonref(&e.ty);
                        if et.t == ty.t || et.is_unknown() {
                            return e;
                        }
                        if let Ty::Class(sc) = et.t {
                            if self.is_base_of(c, sc) {
                                return e;
                            }
                        }
                        if simple_aggr && !self.class_uncertain(c) && form == InitForm::Copy {
                            self.error(line, "INIT-7", "no conversion to an aggregate class");
                            return Expr::unknown(line, "bad init");
                        }
                        if form == InitForm::Copy && !self.classes[c].ctors.is_empty() && !matches!(et.t, Ty::Class(_)) {
                            let x = self.ctor_call(c, std::slice::from_ref(&e), ty, line);
                            self.check_explicit_copy(&x, line);
                            return x;
                        }
                        Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line)
                    }
                    (Some(v), false) if (is_list || is_paren) && !self.classes[c].ctors.is_empty() => {
                        if v.len() == 1 && nonref(&v[0].ty).t == Ty::Class(c) {
                            return Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line);
                        }
                        let x = self.ctor_call(c, v, ty, line);
                        if form == InitForm::CopyList {
                            self.check_explicit_copy(&x, line);
                        }
                        x
                    }
                    _ => Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line),
                }
            }
            _ => {
                // scalar
                let target = ty.unqual();
                match items {
                    Some(v) if is_list || form == InitForm::None => {
                        if v.is_empty() {
                            // INIT-2: empty braces give zero.
                            return Expr::new(EK::ZeroInit, target, Cat::PR, line);
                        }
                        if v.len() > 1 {
                            // LIST-1: at most one expression.
                            self.error(line, "LIST-11", "braced initializer for a scalar with more than one value");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        let x = v[0].clone();
                        if matches!(x.k, EK::InitList(_)) {
                            self.error(line, "LIST-11", "nested braces in a scalar initializer");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        // ENUM-8: an enumeration from an integer in braces.
                        if let Ty::Enum(en) = &target.t {
                            let xt = nonref(&x.ty).t;
                            if xt != target.t && !xt.contains_unknown() && (xt.is_integral() || self.is_unscoped_enum(&xt)) {
                                let Some(fixed) = self.enums[*en].fixed.clone() else {
                                    self.error(line, "ENUM-19", "enumeration without a fixed underlying type initialized from an integer");
                                    return Expr::unknown(line, "bad enum init");
                                };
                                if form != InitForm::DirectList {
                                    self.error(line, "ENUM-19", "copy-list-initialization of an enumeration from an integer");
                                    return Expr::unknown(line, "bad enum init");
                                }
                                if self.is_narrowing(&x, &fixed) {
                                    self.error(line, "LIST-2", "narrowing conversion to the underlying type");
                                }
                                let x = self.rvalue(x);
                                return Expr::new(EK::Conv(Conv::Arith, Box::new(x)), target, Cat::PR, line);
                            }
                        }
                        if self.is_narrowing(&x, &target.t) {
                            // LIST-2: no narrowing conversion in braces.
                            self.error(line, "LIST-2", format!("narrowing conversion from {} to {} in braces", show(&nonref(&x.ty)), show(&target)));
                        }
                        let direct = form == InitForm::DirectList;
                        match self.implicit(x, &target, direct) {
                            Ok(c) => c,
                            Err(m) => {
                                self.error(line, "INIT-7", m);
                                Expr::unknown(line, "bad init")
                            }
                        }
                    }
                    Some(v) => {
                        // parentheses
                        if v.len() != 1 {
                            self.error(line, "INIT-7", "parenthesized initializer for a scalar must hold exactly one expression");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        let x = v[0].clone();
                        if matches!(x.k, EK::InitList(_)) {
                            self.error(line, "INIT-7", "braced list in parentheses for a scalar");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        match self.implicit(x, &target, true) {
                            Ok(c) => c,
                            Err(m) => {
                                self.error(line, "INIT-7", m);
                                Expr::unknown(line, "bad init")
                            }
                        }
                    }
                    None => match self.implicit(e, &target, false) {
                        Ok(c) => c,
                        Err(m) => {
                            self.error(line, "INIT-7", m);
                            Expr::unknown(line, "bad init")
                        }
                    },
                }
            }
        }
    }

    /// VIRT-18: the conditions on a dynamic_cast.
    fn check_dynamic_cast(&mut self, e: &Expr, ty: &QTy, line: u32) {
        let et = nonref(&e.ty);
        // (target class or void, operand class) with their qualifiers
        let (tq, oq) = match &ty.t {
            Ty::Ptr(t) => {
                let Ty::Ptr(o) = &et.t else {
                    self.error(line, "VIRT-18", "dynamic_cast to a pointer needs a pointer operand");
                    return;
                };
                ((**t).clone(), (**o).clone())
            }
            Ty::LRef(t) | Ty::RRef(t) => {
                if e.cat == Cat::PR && matches!(ty.t, Ty::LRef(_)) {
                    self.error(line, "VIRT-18", "dynamic_cast to a reference needs an lvalue or xvalue operand");
                    return;
                }
                ((**t).clone(), et.clone())
            }
            _ => {
                self.error(line, "VIRT-18", "dynamic_cast target must be a pointer or reference to a class, or void*");
                return;
            }
        };
        let tc = match tq.t {
            Ty::Class(c) => Some(c),
            Ty::Void if matches!(ty.t, Ty::Ptr(_)) => None,
            _ => {
                self.error(line, "VIRT-18", "dynamic_cast target must be a pointer or reference to a class, or void*");
                return;
            }
        };
        if let Some(c) = tc {
            if !self.classes[c].complete {
                self.error(line, "VIRT-18", "dynamic_cast to an incomplete class");
                return;
            }
        }
        if (oq.c && !tq.c) || (oq.v && !tq.v) {
            self.error(line, "VIRT-18", "dynamic_cast removes a cv-qualifier");
            return;
        }
        let Ty::Class(oc) = oq.t else { return };
        if self.class_uncertain(oc) || tc.is_some_and(|c| self.class_uncertain(c)) {
            return;
        }
        match tc {
            Some(c) if c == oc => {}
            Some(c) if self.is_base_of(c, oc) => {
                let (paths, open) = self.base_paths(c, oc);
                if paths > 1 {
                    self.error(line, "VIRT-18", "dynamic_cast to an ambiguous base");
                } else if !open {
                    self.error(line, "VIRT-18", "dynamic_cast to an inaccessible base");
                }
            }
            _ => {
                if !self.polymorphic(oc) {
                    self.error(line, "VIRT-18", "dynamic_cast operand of a class that is not polymorphic");
                }
            }
        }
    }

    /// The number of non-virtual paths from `derived` to `base`, and
    /// whether one of them goes only through public bases.
    fn base_paths(&self, base: usize, derived: usize) -> (u32, bool) {
        let mut n = 0;
        let mut open = false;
        let mut virt_seen = false;
        for b in &self.classes[derived].bases {
            let Some(bc) = b.class else { continue };
            let (k, o) = if bc == base { (1, true) } else { self.base_paths(base, bc) };
            if k > 0 {
                if b.virt && bc == base {
                    if virt_seen {
                        continue;
                    }
                    virt_seen = true;
                }
                n += k;
                open |= o && b.access == 0;
            }
        }
        (n, open)
    }

    fn polymorphic(&self, c: usize) -> bool {
        self.classes[c].has_virtual || self.classes[c].bases.iter().any(|b| b.class.is_some_and(|bc| self.polymorphic(bc)))
    }

    /// CTOR-9: copy-initialization does not use an explicit constructor.
    fn check_explicit_copy(&mut self, x: &Expr, line: u32) {
        if let EK::Construct(cid, _) = &x.k {
            if self.func(*cid).is_some_and(|f| f.explicit) {
                self.error(line, "CTOR-9", "copy-initialization uses an explicit constructor");
            }
        }
    }

    /// The default constructor of a class, when exactly one constructor
    /// takes no arguments and none is a template.
    pub fn default_ctor(&self, c: usize) -> Option<DeclId> {
        let ctors = &self.classes[c].ctors;
        if ctors.iter().any(|&i| self.func(i).is_none_or(|f| f.template)) {
            return None;
        }
        let v: Vec<DeclId> = ctors.iter().copied().filter(|&i| self.func(i).is_some_and(|f| f.params.len() <= f.defaults)).collect();
        if v.len() == 1 {
            Some(v[0])
        } else {
            None
        }
    }

    /// A constructor call with the given arguments. The front end picks a
    /// constructor only when every candidate of the right arity has scalar
    /// parameters and exactly one accepts the arguments; otherwise the
    /// call stays unknown.
    pub fn ctor_call(&mut self, c: usize, args: &[Expr], ty: &QTy, line: u32) -> Expr {
        let unknown = Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line);
        if self.class_uncertain(c) || self.in_template() || args.iter().any(|a| a.ty.is_unknown() || matches!(a.k, EK::InitList(_))) {
            return unknown;
        }
        let mut viable = Vec::new();
        for cid in self.classes[c].ctors.clone() {
            let Some(f) = self.func(cid).cloned() else { return unknown };
            if f.template {
                return unknown;
            }
            let np = f.params.len();
            if args.len() > np || args.len() + f.defaults < np {
                continue;
            }
            let mut conv = Vec::new();
            for (i, a) in args.iter().enumerate() {
                let pt = self.var(f.params[i]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                if pt.is_unknown() || pt.t.is_ref() || matches!(pt.t, Ty::Class(_) | Ty::Array(..)) || matches!(nonref(&a.ty).t, Ty::Class(_)) {
                    return unknown;
                }
                match self.implicit(a.clone(), &pt.unqual(), false) {
                    Ok(x) => conv.push(x),
                    Err(_) => break,
                }
            }
            if conv.len() == args.len() {
                if f.deleted {
                    return unknown;
                }
                viable.push((cid, conv));
            }
        }
        if viable.len() != 1 {
            return unknown;
        }
        let (cid, conv) = viable.pop().unwrap();
        Expr::new(EK::Construct(cid, conv), ty.unqual(), Cat::PR, line)
    }

    pub fn is_simple_aggregate(&self, c: usize) -> bool {
        let ci = &self.classes[c];
        ci.complete && !ci.opaque && !ci.template && ci.ctors.is_empty() && ci.bases.is_empty() && !ci.has_virtual
    }

    /// Placeholder type deduction (AUTO-1 to AUTO-11).
    pub fn deduce_auto(&mut self, ds: &DeclSpec, d: &Declarator, _ty: &QTy, e: &Expr, form: &InitForm, line: u32) -> Option<QTy> {
        let decltype_auto = ds.auto == Some(true);
        let mut init = e.clone();
        match form {
            InitForm::CopyList => {
                if decltype_auto {
                    self.error(line, "AUTO-11", "decltype(auto) with '= { ... }'");
                    return None;
                }
                // AUTO-7: std::initializer_list, a library type.
                if let EK::InitList(v) = &e.k {
                    if v.is_empty() {
                        self.error(line, "AUTO-21", "auto deduced from an empty braced list");
                    }
                }
                return Some(QTy::unknown());
            }
            InitForm::DirectList | InitForm::Paren(_) => {
                let EK::InitList(v) = &e.k else { return None };
                if v.len() != 1 {
                    self.error(line, if decltype_auto { "AUTO-11" } else { "AUTO-20" }, "placeholder initializer must contain exactly one expression");
                    return None;
                }
                if matches!(v[0].k, EK::InitList(_)) {
                    self.error(line, "AUTO-20", "braced list in parentheses for a placeholder");
                    return None;
                }
                init = v[0].clone();
            }
            _ => {}
        }
        if matches!(init.k, EK::Overloads(_)) {
            self.error(line, "AUTO-8", "auto initialized from an overloaded function name");
            return None;
        }
        if init.ty.is_unknown() {
            return Some(QTy::unknown());
        }
        if init.ty.t == Ty::Void {
            self.error(line, "AUTO-8", "auto initialized from a void expression");
            return None;
        }
        if decltype_auto {
            if !d.ops.is_empty() {
                self.error(line, "AUTO-11", "decltype(auto) combined with a declarator operator");
                return None;
            }
            if matches!(init.k, EK::Func(_)) && !init.paren {
                self.error(line, "AUTO-11", "decltype(auto) initialized from a function name");
                return None;
            }
            let t = self.decltype_of(&init, init.paren);
            if let Ty::Array(..) = t.t {
                if matches!(init.k, EK::Var(_)) {
                    // AUTO-15
                    self.error(line, "AUTO-15", "decltype(auto) deduces an array type");
                    return None;
                }
            }
            return Some(t);
        }
        let cvq = (ds.c > 0, ds.v > 0);
        let et = nonref(&init.ty);
        let ops = &d.ops;
        let rebuild = |p: &mut P, base: QTy| -> QTy {
            let mut b = base;
            b.c |= cvq.0;
            b.v |= cvq.1;
            p.apply_ops(b, ops, line)
        };
        if ops.is_empty() {
            // AUTO-1, AUTO-2: drop reference and top-level cv, decay.
            let t = match &et.t {
                Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(et.clone()))),
                _ => et.unqual(),
            };
            return Some(rebuild(self, t));
        }
        match &ops[..] {
            [DOp::LRef] => {
                // AUTO-3
                let mut t = et.clone();
                t.c |= cvq.0;
                if init.cat != Cat::L && !t.c {
                    self.error(line, "AUTO-18", "auto& bound to an rvalue");
                    return None;
                }
                Some(QTy::new(Ty::LRef(Box::new(t))))
            }
            [DOp::RRef] => {
                // AUTO-4: forwarding reference
                if cvq.0 || cvq.1 {
                    let mut t = et.clone();
                    t.c |= cvq.0;
                    if init.cat == Cat::L {
                        self.error(line, "REF-10", "const auto&& bound to an lvalue");
                        return None;
                    }
                    return Some(QTy::new(Ty::RRef(Box::new(t))));
                }
                if init.cat == Cat::L {
                    Some(QTy::new(Ty::LRef(Box::new(et.clone()))))
                } else {
                    Some(QTy::new(Ty::RRef(Box::new(et.clone()))))
                }
            }
            [DOp::Ptr(c, v)] => {
                // AUTO-5: auto* needs a pointer
                let pt = match &et.t {
                    Ty::Ptr(p) => (**p).clone(),
                    Ty::Array(el, _) => (**el).clone(),
                    Ty::Func(_) => et.clone(),
                    Ty::NullPtr | _ => {
                        self.error(line, "AUTO-19", "auto* initialized from a value that is not a pointer");
                        return None;
                    }
                };
                let mut pt = pt;
                pt.c |= cvq.0;
                pt.v |= cvq.1;
                Some(QTy { t: Ty::Ptr(Box::new(pt)), c: *c, v: *v })
            }
            _ => {
                if ops.iter().any(|o| matches!(o, DOp::Array(_))) {
                    self.error(line, "AUTO-8", "array of auto");
                    return None;
                }
                Some(QTy::unknown())
            }
        }
    }
}

/// A braced-init-list as written, before it initializes anything.
pub fn is_raw_list(e: &Expr) -> bool {
    matches!(e.k, EK::InitList(_)) && e.ty.is_unknown()
}

pub fn ret_cat(r: &QTy) -> (QTy, Cat) {
    match &r.t {
        Ty::LRef(i) => ((**i).clone(), Cat::L),
        Ty::RRef(i) => ((**i).clone(), if matches!(i.t, Ty::Func(_)) { Cat::L } else { Cat::X }),
        _ => (if matches!(r.t, Ty::Class(_)) { r.clone() } else { r.unqual() }, Cat::PR),
    }
}
