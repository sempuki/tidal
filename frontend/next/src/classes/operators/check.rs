//! The check part of `classes/operators`, for build step 4 part a:
//! operator functions that are members or non-members and not templates
//! (OPR-1, OPR-2), and the selection of an operator function for an
//! expression with an operand of class or enumeration type (OPR-6,
//! OPR-15), and literal operators that are not templates (OPR-8 to
//! OPR-10, OPR-16). `operator->` and `operator co_await` are unsupported.

use crate::expressions::check::bad;
use crate::overloading::conversion_sequences::check::{better, sequence, Ics};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, UOp, EK};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{EntKind, EntityId};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The name of the operator function for an operator's spelling.
pub fn name_of(op: &str) -> Option<&'static str> {
    Some(match op {
        "co_await" => "operatorco_await",
        "->" => "operator->",
        "->*" => "operator->*",
        "~" => "operator~",
        "!" => "operator!",
        "+" => "operator+",
        "-" => "operator-",
        "*" => "operator*",
        "/" => "operator/",
        "%" => "operator%",
        "^" => "operator^",
        "&" => "operator&",
        "|" => "operator|",
        "=" => "operator=",
        "+=" => "operator+=",
        "-=" => "operator-=",
        "*=" => "operator*=",
        "/=" => "operator/=",
        "%=" => "operator%=",
        "^=" => "operator^=",
        "&=" => "operator&=",
        "|=" => "operator|=",
        "==" => "operator==",
        "!=" => "operator!=",
        "<" => "operator<",
        ">" => "operator>",
        "<=" => "operator<=",
        ">=" => "operator>=",
        "<=>" => "operator<=>",
        "&&" => "operator&&",
        "||" => "operator||",
        "<<" => "operator<<",
        ">>" => "operator>>",
        "<<=" => "operator<<=",
        ">>=" => "operator>>=",
        "++" => "operator++",
        "--" => "operator--",
        "," => "operator,",
        "()" => "operator()",
        "[]" => "operator[]",
        "new" => "operatornew",
        "delete" => "operatordelete",
        "new[]" => "operatornew[]",
        "delete[]" => "operatordelete[]",
        _ => return None,
    })
}

/// The name of an `operator_function_id` node.
pub fn function_name(cx: &Check, n: &Node) -> Option<&'static str> {
    let o = n.child("overloadable_operator")?;
    let spelling: String = o.kids.iter().map(|k| cx.text(k)).collect();
    name_of(&spelling)
}

/// The name of an `unqualified_id`: its identifier, or the name of its
/// operator function.
pub fn unqualified_name<'a>(cx: &Check<'a>, u: &Node) -> Option<&'a str> {
    match u.kids.as_slice() {
        [k @ Kid::Tok(_)] => Some(cx.text(k)),
        [Kid::Node(o)] if o.prod == "operator_function_id" => function_name(cx, o),
        _ => None,
    }
}

/// Whether a name is that of an operator function.
pub fn is_operator(name: &str) -> bool {
    name.starts_with("operator") && name_of(&name["operator".len()..]).is_some()
}

/// OPR-2: the limits on the declaration of an operator function. `class`
/// is the class of a member. True when the declaration is valid; an
/// unsupported form gives false after its diagnostic too.
pub fn declaration(cx: &mut Check, name: &str, ty: &QType, defaults: bool, class: Option<crate::sem::types::ClassId>, is_static: bool, explicit_object: bool, span: Span) -> bool {
    let op = &name["operator".len()..];
    // The allocation and deallocation functions have the rules of
    // `expressions/new-delete`.
    if matches!(op, "new" | "delete" | "new[]" | "delete[]") {
        return true;
    }
    if op == "co_await" {
        cx.unsupported(span, format!("the operator function '{}'", name));
        return false;
    }
    let Some(f) = ty.func() else { return true };
    let any = matches!(op, "()" | "[]");
    let bad = |cx: &mut Check, why: String| {
        let cause = (op == "=").then_some("CPY-16");
        cx.error_cause(span, "OPR-2", cause, why);
        false
    };
    if f.variadic && !(op == "()") {
        return bad(cx, format!("the operator function '{}' is variadic", name));
    }
    if defaults && !any {
        return bad(cx, format!("the operator function '{}' has a default argument", name));
    }
    if is_static && class.is_some() && !any {
        return bad(cx, format!("the operator function '{}' is static", name));
    }
    if class.is_none() && matches!(op, "=" | "()" | "[]" | "->") {
        return bad(cx, format!("the operator function '{}' must be a member", name));
    }
    if class.is_none() {
        let user_type = |t: &QType| {
            let t = t.nonref();
            t.is_class() || t.is_enum()
        };
        if !f.params.iter().any(user_type) {
            return bad(cx, format!("the operator function '{}' has no parameter of class or enumeration type", name));
        }
    }
    if any {
        return true;
    }
    // An explicit object parameter is the first operand.
    let count = f.params.len() + usize::from(class.is_some() && !is_static && !explicit_object);
    let ok = match op {
        "!" | "~" | "->" => count == 1,
        "++" | "--" => count == 1 || (count == 2 && f.params.last().is_some_and(|p| p.t == Type::int())),
        "+" | "-" | "*" | "&" => count == 1 || count == 2,
        _ => count == 2,
    };
    if !ok {
        if matches!(op, "++" | "--") && count == 2 {
            return bad(cx, format!("the parameter of the postfix '{}' is not int", op));
        }
        return bad(cx, format!("the operator function '{}' has {} operands", name, count));
    }
    true
}

/// A candidate operator function: a member, whose object is the first
/// argument, or a non-member. A reversed candidate takes the operands in
/// the other order, and a negated one gives the opposite of `==`
/// (rewritten candidates of `==` and `!=`).
#[derive(Clone, Copy)]
struct Cand {
    f: EntityId,
    member: bool,
    reversed: bool,
    negated: bool,
    /// A candidate rewritten from `<=>` (DCMP-6): `(x <=> y) @ 0`, or
    /// `0 @ (y <=> x)` when reversed.
    three: bool,
}

/// The operator functions named `name` that an expression with the
/// operands `args` finds: the members of the first operand's class, and
/// the non-members that ordinary and argument-dependent lookup find.
fn found(cx: &mut Check, name: &str, args: &[Expr], members_of: Option<&Expr>) -> (Vec<EntityId>, Vec<EntityId>) {
    let mut members = Vec::new();
    if let Some(c) = members_of.and_then(|x| x.ty.nonref().class()) {
        if cx.u.classes[c].complete {
            // DER-3: the members of the class or of its bases.
            members = match crate::classes::derived::check::member_lookup(cx, c, name) {
                Ok(Some(x)) => x.ents,
                _ => Vec::new(),
            };
            members.retain(|&e| cx.u.func(e).is_some());
        }
    }
    let mut others: Vec<EntityId> = crate::foundations::names::check::lookup(cx, name).into_iter().filter(|&e| cx.u.func(e).is_some_and(|f| f.class.is_none())).collect();
    let tys: Vec<QType> = args.iter().map(|a| a.ty.clone()).collect();
    for e in crate::foundations::names::check::argument_dependent(cx, name, &tys) {
        if !others.contains(&e) && cx.u.func(e).is_some_and(|f| f.class.is_none()) {
            others.push(e);
        }
    }
    (members, others)
}

/// The conversion sequences of a candidate for its arguments, or None
/// when it is not viable.
fn sequences(cx: &Check, c: &Cand, args: &[Expr]) -> Option<Vec<Ics>> {
    let func = cx.u.func(c.f)?;
    let params = &func.ty.params;
    let mut out = Vec::new();
    // ICS-41: the object meets an explicit object parameter as any
    // argument does.
    let rest = if c.member && func.explicit_object {
        args
    } else if c.member && !func.member_static {
        out.push(crate::overloading::overload_resolution::check::object_sequence(cx, c.f, &args[0])?);
        &args[1..]
    } else if c.member {
        &args[1..]
    } else {
        args
    };
    let required = func.defaults.iter().take_while(|d| d.is_none()).count().min(params.len());
    if rest.len() > params.len() || rest.len() < required {
        return None;
    }
    for (k, (a, p)) in rest.iter().zip(params.iter()).enumerate() {
        match sequence(cx, a, p) {
            Some(s) => out.push(s),
            // ICS-4: a converting constructor makes the parameter's object;
            // the object of a member takes no such conversion (OPR-6).
            // A conversion function of the operand's class to the
            // parameter's class (ICS-4).
            None if !(c.member && k == 0 && func.member_static && !func.explicit_object) => out.push(user_conversion(cx, a, p).or_else(|| crate::classes::conversion_functions::check::user_sequence(cx, a, p).filter(|_| p.nonref().is_class()))?),
            None => return None,
        }
    }
    Some(out)
}

/// A user-defined conversion sequence from `a` to a parameter of class
/// type, or a reference to const of it: a converting constructor that
/// takes `a` by a standard sequence.
pub fn user_conversion(cx: &Check, a: &Expr, p: &QType) -> Option<Ics> {
    if p.is_ref() && !(p.nonref().c && !p.nonref().v) && !matches!(p.t, Type::Ref(_, true)) {
        return None;
    }
    let c = p.nonref().class()?;
    // A conversion function of the source class to the class competes
    // with the constructors, which a later piece weighs.
    if a.ty.nonref().class() == Some(c) || crate::classes::conversion_functions::check::reaches_class(cx, a, c) {
        return None;
    }
    let ok = crate::classes::constructors::check::template_takes(cx, c, a) || crate::classes::constructors::check::ctors(cx, c).into_iter().any(|f| {
        cx.u.func(f).is_some_and(|x| !x.explicit && !x.deleted && !x.ty.params.is_empty() && (1..x.ty.params.len()).all(|k| x.defaults.get(k).is_some_and(|d| d.is_some())) && sequence(cx, a, &x.ty.params[0]).is_some_and(|i| i.user.is_none() && i.rank != crate::overloading::conversion_sequences::check::Rank::User))
    });
    ok.then(|| Ics { rank: crate::overloading::conversion_sequences::check::Rank::User, qual: false, binds: None, ellipsis: false, user: None, extra: Default::default() })
}

/// OPR-15: the best viable candidate, or None after an error.
fn choose(cx: &mut Check, cands: &[Cand], args: &[Expr], span: Span, op: &str) -> Option<Cand> {
    let mut viable: Vec<(Cand, Vec<Ics>)> = Vec::new();
    for c in cands {
        let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.to_vec() };
        if let Some(s) = sequences(cx, c, &ordered) {
            // Compare the sequences in the order of the operands.
            let s = if c.reversed { s.into_iter().rev().collect() } else { s };
            viable.push((*c, s));
        }
    }
    if viable.is_empty() {
        cx.error_cause(span, "OPR-15", Some("OVL-35"), format!("no operator function '{}' fits the operands", op));
        return None;
    }
    let wins = |a: &(Cand, Vec<Ics>), b: &(Cand, Vec<Ics>)| -> bool {
        let mut some = false;
        for (x, y) in a.1.iter().zip(b.1.iter()) {
            if better(y, x) {
                return false;
            }
            if better(x, y) {
                some = true;
            }
        }
        // A candidate that is not rewritten wins a tie, and a rewritten
        // one wins a tie with a reversed one (OVL rules).
        let rewritten = |c: &Cand| c.reversed || c.negated || c.three;
        some || (!rewritten(&a.0) && rewritten(&b.0)) || (!a.0.reversed && b.0.reversed)
    };
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i], &viable[best]) {
            best = i;
        }
    }
    for i in 0..viable.len() {
        if i != best && !wins(&viable[best], &viable[i]) {
            cx.error_cause(span, "OPR-15", Some("OVL-36"), format!("the operator '{}' is ambiguous for these operands", op));
            return None;
        }
    }
    Some(viable[best].0)
}

/// The call of the chosen operator function with the operands.
fn build(cx: &mut Check, c: Cand, args: Vec<Expr>, span: Span) -> Expr {
    let op = cx.u.entities[c.f].name.clone();
    let mut call = build_call(cx, c, args, span);
    // OPR-7: the right operand of an assignment is evaluated first.
    if matches!(&op["operator".len().min(op.len())..], "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=" | "&=" | "|=" | "<<=" | ">>=") {
        if let EK::Call(_, _, rev) = &mut call.k {
            *rev = true;
        }
    }
    call
}

fn build_call(cx: &mut Check, c: Cand, args: Vec<Expr>, span: Span) -> Expr {
    let args: Vec<Expr> = if c.reversed { args.into_iter().rev().collect() } else { args };
    let func = cx.u.func(c.f).unwrap().clone();
    if c.member && !crate::classes::members::check::accessible(cx, c.f) {
        cx.error(span, "ACC-2", format!("the member '{}' is not public", cx.u.entities[c.f].name));
        return bad(span);
    }
    if func.deleted {
        cx.error(span, "DEL-1", format!("the operator function '{}' is deleted", cx.u.entities[c.f].name));
        return bad(span);
    }
    crate::declarations::functions::definitions::check::note_use(cx, c.f, span);
    // VIRT-1: a virtual operator function calls its final overrider.
    let kind = if c.member && cx.u.virt.contains_key(&c.f) { EK::Virtual(c.f) } else { EK::Func(c.f) };
    let callee = Expr::new(kind, QType::new(Type::Func(func.ty.clone())), Cat::L, span);
    let call = if c.member {
        let mut args = args;
        let mut obj = args.remove(0);
        // DER-3: the object of a member of a base is the base subobject.
        if let (Some(oc), Some(k)) = (obj.ty.nonref().class(), func.class) {
            if oc != k {
                let o = crate::foundations::values::check::deref_ref(obj);
                let ps = crate::classes::derived::check::paths(cx, oc, k);
                if ps.len() != 1 {
                    cx.error(span, "DER-4", format!("the operator function '{}' is in more than one base subobject", cx.u.entities[c.f].name));
                    return bad(span);
                }
                obj = crate::classes::derived::check::along(cx, o, oc, &ps[0]);
            }
        }
        // CLS-13: the object is the first argument of a function with an
        // explicit object parameter.
        let obj = if func.explicit_object {
            args.insert(0, obj);
            None
        } else if func.member_static {
            None
        } else {
            Some(obj)
        };
        crate::expressions::postfix::check::finish_call(cx, callee, Some(c.f), args, span, obj)
    } else {
        crate::expressions::postfix::check::finish_call(cx, callee, Some(c.f), args, span, None)
    };
    if c.negated && !call.ty.is_error() {
        let b = crate::foundations::conversions::check::contextual_bool(cx, call, "OPR-6");
        return Expr::new(EK::Unary(UOp::Not, Box::new(b)), QType::bool(), Cat::PR, span);
    }
    call
}

/// OVL-13, LSUP-12: the comparison of the result of a rewritten `<=>`
/// with a literal 0: `v @ 0`, or `0 @ v` for a reversed candidate.
pub fn compare_zero(cx: &mut Check, op: &str, v: Expr, reversed: bool, span: Span) -> Expr {
    if v.ty.is_error() {
        return v;
    }
    let zero = Expr::int(0, QType::int(), span);
    let (l, r) = if reversed { (zero, v) } else { (v, zero) };
    if let Some(e) = binary_with(cx, op, &l, &r, span, false) {
        return e;
    }
    if l.ty.nonref().is_class() || r.ty.nonref().is_class() {
        cx.error_cause(span, "OVL-43", Some("OVL-35"), format!("no operator '{}' compares the result of the rewritten three-way comparison with 0", op));
        return bad(span);
    }
    match crate::sem::tree::BOp::from_str(op) {
        Some(crate::sem::tree::BOp::Spaceship) => crate::expressions::comparison::check::three_way(cx, l, r, span),
        Some(b) => crate::expressions::comparison::check::operate(cx, b, l, r, span),
        None => bad(span),
    }
}

fn user_type(t: &QType) -> bool {
    let t = t.nonref();
    t.is_class() || t.is_enum()
}

/// A binary operator `op` with an operand of class or enumeration type
/// (OPR-6): the call of the operator function that overload resolution
/// chooses. None when the built-in operator applies: no operand of such
/// a type, or no operator function for an enumeration operand.
pub fn binary(cx: &mut Check, op: &str, l: &Expr, r: &Expr, span: Span) -> Option<Expr> {
    binary_with(cx, op, l, r, span, true)
}

/// `binary`, with the rewritten and reversed candidates of DCMP-6 when
/// `rewrite`. The comparison with 0 of a rewritten `<=>` has none
/// (OVL-43).
fn binary_with(cx: &mut Check, op: &str, l: &Expr, r: &Expr, span: Span, rewrite: bool) -> Option<Expr> {
    if l.ty.is_error() || r.ty.is_error() || !(user_type(&l.ty) || user_type(&r.ty)) {
        return None;
    }
    let name = name_of(op)?;
    let args = vec![l.clone(), r.clone()];
    let (members, others) = found(cx, name, &args, Some(l));
    // DEP-9: in a specialization, argument-dependent lookup at the point
    // of instantiation can find an operator declared after the template.
    if cx.lookup_limit.is_some() && crate::foundations::names::check::late_argument_dependent(cx, name, &[l.ty.clone(), r.ty.clone()]) {
        let t = cx.unsupported(span, format!("an operator '{}' in a specialization that argument-dependent lookup after the template can find", op));
        return Some(Expr::new(EK::Int(0), t, crate::sem::tree::Cat::PR, span));
    }
    let mut cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false, three: false }).chain(others.iter().map(|&f| Cand { f, member: false, reversed: false, negated: false, three: false })).collect();
    // The rewritten candidates of `==` and `!=`: `y == x`, and the
    // opposite of `x == y` and `y == x`.
    let mut excluded = false;
    if rewrite && matches!(op, "==" | "!=") {
        let (m1, o1) = found(cx, "operator==", &args, Some(l));
        let (m2, _) = found(cx, "operator==", &args, Some(r));
        // DCMP-6, DCMP-13: an `==` with a matching `!=` rewrites nothing.
        let before = m1.len() + o1.len() + m2.len();
        let m1: Vec<EntityId> = m1.into_iter().filter(|&f| !has_matching_ne(cx, f)).collect();
        let o1: Vec<EntityId> = o1.into_iter().filter(|&f| !has_matching_ne(cx, f)).collect();
        let m2: Vec<EntityId> = m2.into_iter().filter(|&f| !has_matching_ne(cx, f)).collect();
        excluded = m1.len() + o1.len() + m2.len() < before;
        let negated = op == "!=";
        if negated {
            cands.extend(m1.iter().map(|&f| Cand { f, member: true, reversed: false, negated: true, three: false }));
            cands.extend(o1.iter().map(|&f| Cand { f, member: false, reversed: false, negated: true, three: false }));
        }
        cands.extend(m2.iter().map(|&f| Cand { f, member: true, reversed: true, negated, three: false }));
        cands.extend(o1.iter().map(|&f| Cand { f, member: false, reversed: true, negated, three: false }));
    }
    if excluded && !cands.iter().any(|c| {
        let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.clone() };
        sequences(cx, c, &ordered).is_some()
    }) {
        cx.error_cause(span, "DCMP-12", Some("DCMP-13"), format!("the operator '{}' needs an operator== that a matching operator!= keeps from rewriting", op));
        return Some(bad(span));
    }
    // DCMP-6: `y <=> x` reversed for `<=>`, and `x <=> y` and `y <=> x`
    // for a relational operator; `<=>` never rewrites `==` or `!=`.
    if rewrite && matches!(op, "<=>" | "<" | ">" | "<=" | ">=") {
        let (m1, o1) = found(cx, "operator<=>", &args, Some(l));
        let (m2, o2) = found(cx, "operator<=>", &args, Some(r));
        if op != "<=>" {
            cands.extend(m1.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false, three: true }));
            cands.extend(o1.iter().map(|&f| Cand { f, member: false, reversed: false, negated: false, three: true }));
        }
        cands.extend(m2.iter().map(|&f| Cand { f, member: true, reversed: true, negated: false, three: true }));
        cands.extend(o2.iter().map(|&f| Cand { f, member: false, reversed: true, negated: false, three: true }));
    }
    // A defaulted comparison is unsupported.
    if cands.iter().any(|c| cx.u.func(c.f).is_some_and(|f| f.unsupported.is_some())) {
        let t = cx.unsupported(span, "an operator function that the front end does not model");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    }
    // OVL-8: with no class operand, a non-member candidate has a
    // parameter of an operand's enumeration type.
    let cands = enum_candidates(cx, cands, &args);
    if cands.is_empty() {
        return None;
    }
    // OVL-9: the built-in candidate that the conversion functions of the
    // operands reach competes with the operator functions.
    match against_builtin(cx, op, &cands, &args, span) {
        Compete::Builtin => return None,
        Compete::Failed => return Some(bad(span)),
        Compete::Unsupported => {
            let t = cx.unsupported(span, "an operator with operator functions and an operand whose class has several conversion functions");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
        Compete::Functions => {}
    }
    let class = l.ty.nonref().is_class() || r.ty.nonref().is_class();
    if !class {
        // An enumeration operand: the built-in operator is a candidate
        // too, which the front end weighs only against an exact match.
        let exact = cands.iter().filter(|c| {
            let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.clone() };
            sequences(cx, c, &ordered).is_some_and(|s| s.iter().all(|x| x.rank == crate::overloading::conversion_sequences::check::Rank::Exact))
        }).count();
        let any = cands.iter().any(|c| {
            let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.clone() };
            sequences(cx, c, &ordered).is_some()
        });
        if !any {
            return None;
        }
        if exact == 0 {
            let t = cx.unsupported(span, "an operator function against a built-in operator for enumeration operands");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
        // OVL-10: a comparison of two operands of one enumeration has the
        // built-in candidate with that type, which ties an operator
        // function whose parameters are references to it. One with the
        // same parameter types replaces it.
        if matches!(op, "==" | "!=" | "<" | ">" | "<=" | ">=") && l.ty.nonref().unqual().t == r.ty.nonref().unqual().t {
            let e = l.ty.nonref().unqual();
            let same = cands.iter().any(|c| !c.reversed && !c.negated && cx.u.func(c.f).is_some_and(|f| f.ty.params.len() == 2 && f.ty.params.iter().all(|p| p.unqual().t == e.t && !p.is_ref())));
            if !same {
                cx.error_cause(span, "OVL-36", Some("OPR-15"), format!("the operator '{}' is ambiguous between an operator function and the built-in operator", op));
                return Some(bad(span));
            }
        }
    }
    // `&` and `,` with no viable operator function are built in.
    if matches!(op, "," | "&&" | "||") && !cands.iter().any(|c| sequences(cx, c, &args).is_some()) && !class {
        return None;
    }
    let c = choose(cx, &cands, &args, span, op)?;
    if c.three {
        let v = build(cx, c, args, span);
        return Some(compare_zero(cx, op, v, c.reversed, span));
    }
    // DCMP-12: a rewritten or reversed `==` returns bool.
    if c.negated || c.reversed {
        let ret = cx.u.func(c.f).map(|f| f.ty.ret.clone()).unwrap_or_else(QType::error);
        if ret.is_ref() || !ret.unqual().is_bool() {
            cx.error(span, "DCMP-12", format!("the operator== that rewrites '{}' returns {}, not bool", op, ret.show()));
            return Some(bad(span));
        }
    }
    Some(build(cx, c, args, span))
}

/// OVL-8: when no operand has class type, a non-member candidate is one
/// with a parameter of an operand's enumeration type or a reference to
/// it. A member candidate has a class operand, so it stays.
fn enum_candidates(cx: &Check, cands: Vec<Cand>, args: &[Expr]) -> Vec<Cand> {
    if args.iter().any(|a| a.ty.nonref().is_class()) {
        return cands;
    }
    let enums: Vec<Type> = args.iter().map(|a| a.ty.nonref().unqual().t.clone()).filter(|t| matches!(t, Type::Enum(_))).collect();
    cands.into_iter().filter(|c| c.member || cx.u.func(c.f).is_some_and(|f| f.ty.params.iter().any(|p| enums.contains(&p.nonref().unqual().t)))).collect()
}

/// The outcome of the built-in candidate against the operator functions
/// (OVL-9).
enum Compete {
    /// No built-in candidate, or the operator functions are better.
    Functions,
    /// The built-in operator is better, or the only viable candidate.
    Builtin,
    /// An error is out.
    Failed,
    /// A choice that the front end does not weigh.
    Unsupported,
}

/// OVL-9: the built-in candidate that the conversion functions of the
/// operands reach, against the best operator function. Each class
/// operand of the built-in candidate converts by a conversion function
/// to a scalar type, a user-defined sequence; an explicit one counts
/// for `!`, `&&`, and `||`, whose operands are contextually converted.
/// The candidate whose user-defined sequences are a strict subset of the
/// other's is better; otherwise the choice is ambiguous (OVL-36).
fn against_builtin(cx: &mut Check, op: &str, cands: &[Cand], args: &[Expr], span: Span) -> Compete {
    use crate::classes::conversion_functions::check::{converting_class, functions};
    if !args.iter().any(|a| converting_class(cx, a).is_some()) {
        return Compete::Functions;
    }
    let logical = matches!(op, "!" | "&&" | "||");
    let mut b_user = Vec::new();
    for a in args {
        let Some(k) = converting_class(cx, a) else {
            if a.ty.nonref().is_class() {
                return Compete::Functions;
            }
            b_user.push(false);
            continue;
        };
        let obj = a.ty.nonref();
        let mut tys: Vec<QType> = Vec::new();
        for f in functions(&cx.u, k) {
            let Some(g) = cx.u.func(f) else { continue };
            let r = crate::declarations::auto::check::decay(&g.ty.ret.nonref()).unqual();
            let scalar = r.is_arithmetic() || r.is_enum() || r.is_ptr() || r.is_member_ptr() || r.is_nullptr();
            if scalar && (logical || !g.explicit) && (!obj.c || g.obj_cv.0) && !tys.contains(&r) {
                tys.push(r);
            }
        }
        match tys.len() {
            0 => return Compete::Functions,
            1 => b_user.push(true),
            _ if logical => b_user.push(true),
            _ => return Compete::Unsupported,
        }
    }
    let viable: Vec<&Cand> = cands.iter().filter(|c| {
        let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.to_vec() };
        sequences(cx, c, &ordered).is_some()
    }).collect();
    if viable.is_empty() {
        return Compete::Builtin;
    }
    let Some(c) = choose(cx, cands, args, span, op) else { return Compete::Failed };
    let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.to_vec() };
    let Some(seqs) = sequences(cx, &c, &ordered) else { return Compete::Functions };
    let mut c_user: Vec<bool> = seqs.iter().map(|s| s.rank == crate::overloading::conversion_sequences::check::Rank::User).collect();
    c_user.truncate(args.len());
    if c.reversed {
        c_user.reverse();
    }
    let subset = |a: &[bool], b: &[bool]| a.iter().zip(b).all(|(x, y)| !*x || *y) && a != b;
    if subset(&b_user, &c_user) {
        return Compete::Builtin;
    }
    if subset(&c_user, &b_user) {
        return Compete::Functions;
    }
    cx.error_cause(span, "OVL-36", Some("OPR-15"), format!("the operator '{}' is ambiguous between an operator function and the built-in operator", op));
    Compete::Failed
}

/// DCMP-13: an `operator!=` that matches the `operator==` `f`: a member
/// of the same class or a non-member of the same namespace, with the
/// same parameter types.
fn has_matching_ne(cx: &Check, f: EntityId) -> bool {
    let Some(func) = cx.u.func(f) else { return false };
    let scope = cx.u.entities[f].scope;
    // For a member, member lookup in its class, which finds a base's.
    let ne = match func.class.and_then(|c| crate::classes::derived::check::member_lookup(cx, c, "operator!=").ok().flatten()) {
        Some(x) => x.ents,
        None => crate::foundations::names::check::declared_here(cx, scope, "operator!="),
    };
    // DCMP-13: ordinary lookup, which does not find a friend that only a
    // class declares (ACC-7).
    ne.into_iter().filter(|g| !cx.u.hidden.contains(g)).any(|g| {
        cx.u.func(g).is_some_and(|h| h.ty.params == func.ty.params && (h.class.is_some() && !h.member_static) == (func.class.is_some() && !func.member_static))
    })
}

/// A unary operator `op` on an operand of class or enumeration type, or a
/// postfix `++` or `--` (`postfix`), which passes `0` (OPR-4).
pub fn unary(cx: &mut Check, op: &str, e: &Expr, postfix: bool, span: Span) -> Option<Expr> {
    if e.ty.is_error() || !user_type(&e.ty) {
        return None;
    }
    let name = name_of(op)?;
    let mut args = vec![e.clone()];
    if postfix {
        args.push(Expr::int(0, QType::int(), span));
    }
    let (members, others) = found(cx, name, &args, Some(e));
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false, three: false }).chain(others.iter().map(|&f| Cand { f, member: false, reversed: false, negated: false, three: false })).collect();
    let cands = enum_candidates(cx, cands, &args[..1]);
    if cands.is_empty() {
        return None;
    }
    match against_builtin(cx, op, &cands, &args[..1], span) {
        Compete::Builtin => return None,
        Compete::Failed => return Some(bad(span)),
        Compete::Unsupported => {
            let t = cx.unsupported(span, "an operator with operator functions and an operand whose class has several conversion functions");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
        Compete::Functions => {}
    }
    if cands.iter().any(|c| cx.u.func(c.f).is_some_and(|f| f.unsupported.is_some())) {
        let t = cx.unsupported(span, "an operator function that the front end does not model");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    }
    let viable = cands.iter().any(|c| sequences(cx, c, &args).is_some());
    // `&` with no viable operator function is built in; so is an
    // enumeration operand.
    if !viable && (op == "&" || !e.ty.nonref().is_class()) {
        return None;
    }
    // An enumeration operand: an operator function that takes it by an
    // exact match beats the built-in operator, which promotes it.
    if !e.ty.nonref().is_class() {
        let exact = cands.iter().any(|c| sequences(cx, c, &args).is_some_and(|s| s.iter().all(|x| x.rank == crate::overloading::conversion_sequences::check::Rank::Exact)));
        if !exact {
            let t = cx.unsupported(span, "an operator function against a built-in operator for an enumeration operand");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
    }
    // OPR-13: `++` and `--` use only the form the class declares.
    if !viable && matches!(op, "++" | "--") {
        cx.error_cause(span, "OPR-13", Some("OPR-15"), format!("no {} form of '{}' fits the operand", if postfix { "postfix" } else { "prefix" }, name));
        return Some(bad(span));
    }
    let c = choose(cx, &cands, &args, span, op)?;
    Some(build(cx, c, args, span))
}

/// `a = b` with a class object on the left: a user-declared assignment
/// operator (CPY-3). None when the implicit copy or move assignment
/// applies: the class declares no copy or move assignment and the right
/// operand is an object of the class.
pub fn assignment(cx: &mut Check, l: &Expr, r: &Expr, span: Span) -> Option<Expr> {
    let c = l.ty.nonref().class()?;
    if r.ty.is_error() {
        return None;
    }
    let members: Vec<EntityId> = crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator=").into_iter().filter(|&e| cx.u.func(e).is_some()).collect();
    if members.iter().all(|f| cx.u.defaulted.contains(f)) {
        // UNI-3: a variant member deletes the implicit assignment of a
        // union. A deleted move assignment gives way to the copy.
        if members.is_empty() && r.ty.nonref().class() == Some(c) {
            use crate::classes::unions::check::{deleting_member, usable, Special};
            let which = if r.cat != Cat::L && deleting_member(cx, c, Special::MoveAssign).is_none() { Special::MoveAssign } else { Special::CopyAssign };
            if !usable(cx, c, which, span) {
                return Some(bad(span));
            }
        }
        // The implicit or defaulted assignment: each member in turn where
        // a member runs code (CPY-8).
        // A union copies its active member (UNI-5).
        if r.ty.nonref().class() == Some(c) && needs_assign(cx, c) && !l.ty.c && !crate::classes::unions::check::is_union(cx, c) {
            return Some(memberwise(cx, l, r, c, span));
        }
        return None;
    }
    let copy_or_move = members.iter().any(|&f| crate::classes::copy_move::check::assignment_kind(cx, c, f).is_some());
    if !copy_or_move && r.ty.nonref().class() == Some(c) {
        return None;
    }
    if members.iter().any(|&f| cx.u.func(f).is_some_and(|x| x.unsupported.is_some())) {
        let t = cx.unsupported(span, "an assignment operator that the front end does not model");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    }
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false, three: false }).collect();
    let args = vec![l.clone(), r.clone()];
    // A defaulted copy or move assignment is the implicit one.
    let c2 = choose(cx, &cands, &args, span, "=")?;
    if cx.u.defaulted.contains(&c2.f) {
        return None;
    }
    Some(build(cx, c2, args, span))
}

/// A member of the class at any depth has a user-provided assignment
/// operator, so an implicit assignment runs code.
pub fn needs_assign(cx: &Check, c: ClassId) -> bool {
    crate::classes::copy_move::check::has_virtual_part(cx, c) || cx.u.field_types(c).iter().any(|t| {
        t.base_elem().class().is_some_and(|d| {
            crate::foundations::names::check::declared_here(cx, cx.u.classes[d].scope, "operator=").iter().any(|f| !cx.u.defaulted.contains(f)) || needs_assign(cx, d)
        })
    })
}

/// The implicit copy or move assignment of `c` (CPY-8): each member
/// assigned in turn, then the left operand. An rvalue is moved only
/// where the class has an implicit move assignment (CPY-5).
fn memberwise(cx: &mut Check, l: &Expr, r: &Expr, c: ClassId, span: Span) -> Expr {
    let effects = |e: &Expr| e.any(&|x| matches!(x.k, EK::Call(..) | EK::Assign(..) | EK::Compound(..) | EK::IncDec(..)));
    if effects(l) || effects(r) {
        let t = cx.unsupported(span, "a memberwise assignment of objects that expressions with side effects name");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    let mut r = crate::foundations::values::check::deref_ref(r.clone());
    if r.cat == Cat::PR {
        let t = r.ty.clone();
        r = r.conv(crate::sem::tree::Conv::Materialize, t, Cat::X);
    }
    if r.cat != Cat::L && !crate::classes::copy_move::check::implicit_move_assign(cx, c) {
        r.cat = Cat::L;
    }
    let l = crate::foundations::values::check::deref_ref(l.clone());
    // The direct virtual bases first, then the members (CPY-18: a virtual
    // base can be assigned more than once).
    let mut pairs: Vec<(Expr, Expr)> = Vec::new();
    for (i, _) in cx.u.bases(c).into_iter().filter(|&(i, _)| i >= crate::sem::VSTEP) {
        pairs.push((crate::classes::derived::check::along(cx, l.clone(), c, &[i]), crate::classes::derived::check::along(cx, r.clone(), c, &[i])));
    }
    for f in cx.u.classes[c].fields.clone() {
        pairs.push((crate::classes::members::check::field_of(cx, l.clone(), f, span), crate::classes::members::check::field_of(cx, r.clone(), f, span)));
    }
    let mut parts: Vec<Expr> = Vec::new();
    for (lm, rm) in pairs {
        if lm.ty.is_array() {
            let t = cx.unsupported(span, "a memberwise assignment of an array member");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        }
        let part = match lm.ty.class() {
            Some(d) => {
                // CPY-22: an assignment operator that takes a non-const
                // reference does not copy a const object.
                if rm.ty.c && rm.cat == Cat::L {
                    let ops = crate::foundations::names::check::declared_here(cx, cx.u.classes[d].scope, "operator=");
                    let copies = ops.iter().filter(|&&g| crate::classes::copy_move::check::assignment_kind(cx, d, g) == Some(false)).collect::<Vec<_>>();
                    if !copies.is_empty() && copies.iter().all(|&&g| cx.u.func(g).is_some_and(|x| !x.ty.params[0].nonref().c && x.ty.params[0].is_ref())) {
                        cx.error(span, "CPY-22", format!("the implicit copy assignment of '{}' takes a non-const reference, which a const object does not bind", cx.u.classes[c].name));
                        return bad(span);
                    }
                }
                match assignment(cx, &lm, &rm, span) {
                    Some(x) => x,
                    None => {
                        let rv = crate::foundations::values::check::rvalue(cx, rm);
                        let t = lm.ty.clone();
                        Expr::new(EK::Assign(Box::new(lm), Box::new(rv)), t, Cat::L, span)
                    }
                }
            }
            None => {
                let rv = crate::foundations::values::check::rvalue(cx, rm);
                let t = lm.ty.clone();
                Expr::new(EK::Assign(Box::new(lm), Box::new(rv)), t, Cat::L, span)
            }
        };
        if part.ty.is_error() {
            return bad(span);
        }
        parts.push(part);
    }
    let mut out = l.clone();
    for p in parts.into_iter().rev() {
        let (t, cat) = (out.ty.clone(), out.cat);
        out = Expr::new(EK::Comma(Box::new(p), Box::new(out)), t, cat, span);
    }
    out
}

/// `a[i]` with an object of class type: its `operator[]` (OPR-3).
pub fn subscript(cx: &mut Check, a: &Expr, args: &[Expr], span: Span) -> Option<Expr> {
    let c = a.ty.nonref().class()?;
    let members: Vec<EntityId> = crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator[]").into_iter().filter(|&e| cx.u.func(e).is_some()).collect();
    if members.is_empty() {
        return None;
    }
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false, three: false }).collect();
    let mut all = vec![a.clone()];
    all.extend(args.iter().cloned());
    let c = choose(cx, &cands, &all, span, "[]")?;
    Some(build(cx, c, all, span))
}

/// The `operator->` functions of class `c` or of its bases (DER-3).
fn arrow_functions(cx: &Check, c: ClassId) -> Vec<EntityId> {
    match crate::classes::derived::check::member_lookup(cx, c, "operator->") {
        Ok(Some(x)) => x.ents,
        _ => Vec::new(),
    }
}

/// OPR-5: the left operand of `x->m` for a class object `x`. Each class
/// object calls its `operator->`, until the result is a pointer to a
/// class, whose `->` is built in. A class with no `operator->`, a result
/// that is neither, and a chain that comes back to a class are errors
/// (OPR-14).
pub fn arrow(cx: &mut Check, b: Expr, span: Span) -> Expr {
    let mut e = crate::foundations::values::check::deref_ref(b);
    let mut seen: Vec<ClassId> = Vec::new();
    while let Some(c) = e.ty.nonref().class() {
        if seen.contains(&c) {
            cx.error(span, "OPR-14", format!("the operator-> of '{}' leads back to an object of {}", cx.u.classes[seen[0]].name, e.ty.nonref().show()));
            return bad(span);
        }
        seen.push(c);
        let fs = arrow_functions(cx, c);
        // PFX-11: the built-in `->` takes a pointer to a class.
        if fs.is_empty() {
            let rule = if seen.len() == 1 { "PFX-11" } else { "OPR-14" };
            cx.error(span, rule, format!("'->' on {}, which is not a pointer to a class and has no operator->", e.ty.nonref().show()));
            return bad(span);
        }
        let Some(f) = crate::overloading::overload_resolution::check::resolve_with(cx, &fs, Some(&e), &[], span, "operator->") else { return bad(span) };
        let Some(func) = cx.u.func(f).cloned() else { return bad(span) };
        let kind = if cx.u.virt.contains_key(&f) { EK::Virtual(f) } else { EK::Func(f) };
        let callee = Expr::new(kind, QType::new(Type::Func(func.ty.clone())), Cat::L, span);
        let r = crate::expressions::postfix::check::finish_call(cx, callee, Some(f), Vec::new(), span, Some(e));
        if r.ty.is_error() {
            return r;
        }
        e = crate::foundations::values::check::deref_ref(r);
    }
    let e = crate::foundations::values::check::rvalue(cx, e);
    if !e.ty.pointee().is_some_and(|p| p.is_class()) {
        cx.error(span, "OPR-14", format!("operator-> gives {}, which is not a pointer to a class", e.ty.show()));
        return bad(span);
    }
    e
}

/// The operator functions named by an entity, for diagnostics.
pub fn is_operator_entity(cx: &Check, e: EntityId) -> bool {
    matches!(cx.u.entities[e].kind, EntKind::Func(_)) && is_operator(&cx.u.entities[e].name)
}

/// The prefix of the name of a literal operator, before its suffix.
pub const LITERAL: &str = "operator\"\"";

/// OPR-9, OPR-10: the limits on the declaration of a literal operator
/// with the suffix `suffix`. `member` is a declaration in a class that is
/// not a friend. True when the declaration is valid.
pub fn literal_declaration(cx: &mut Check, suffix: &str, ty: &QType, defaults: bool, member: bool, span: Span) -> bool {
    if !suffix.starts_with('_') {
        cx.warning(span, "OPR-9", format!("the literal suffix '{}' does not start with '_' and is reserved", suffix));
    }
    let bad = |cx: &mut Check, why: &str| {
        cx.error(span, "OPR-10", why.to_string());
        false
    };
    if member {
        return bad(cx, "a literal operator must not be a member of a class");
    }
    if cx.lang_c {
        return bad(cx, "a literal operator must have C++ language linkage");
    }
    let Some(f) = ty.func() else { return true };
    if f.variadic || literal_form(cx, &f.params).is_none() {
        return bad(cx, "the parameters of a literal operator are not one of its forms");
    }
    if defaults {
        return bad(cx, "a literal operator has a default argument");
    }
    true
}

/// The forms of a literal operator's parameter list (OPR-8).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Form {
    Integer,
    Floating,
    Raw,
    Char(crate::sem::types::IntK),
    Str(crate::sem::types::IntK),
}

fn is_char(k: crate::sem::types::IntK) -> bool {
    use crate::sem::types::IntK::*;
    matches!(k, Char | WChar | Char8 | Char16 | Char32)
}

/// The form that a parameter list has, if any.
fn literal_form(cx: &Check, params: &[QType]) -> Option<Form> {
    use crate::sem::types::{FloatK, IntK};
    let const_char = |t: &QType| match &t.unqual().t {
        Type::Ptr(p) => match p.t {
            Type::Int(k) if p.c && !p.v && is_char(k) => Some(k),
            _ => None,
        },
        _ => None,
    };
    match params {
        [p] => match p.unqual().t {
            Type::Int(IntK::ULongLong) => Some(Form::Integer),
            Type::Float(FloatK::LongDouble) => Some(Form::Floating),
            Type::Int(k) if is_char(k) => Some(Form::Char(k)),
            _ => (const_char(p) == Some(IntK::Char)).then_some(Form::Raw),
        },
        [p, n] if n.unqual().t == Type::Int(cx.u.target.size_t()) => const_char(p).map(Form::Str),
        _ => None,
    }
}

/// A literal with a user-defined suffix (OPR-8, OPR-16): the call of the
/// literal operator for the suffix whose form fits the literal. `toks`
/// are the literal's tokens, more than one for adjacent string literals.
pub fn user_literal(cx: &mut Check, toks: &[&crate::lex::Token], span: Span) -> Expr {
    use crate::lex::Lit;
    use crate::sem::types::{FloatK, IntK};
    let mut suffix: Option<&str> = None;
    for t in toks {
        if let Some(Lit::UserDefined { suffix: s, .. }) = &t.lit {
            if suffix.is_some_and(|x| x != s) {
                cx.error(t.span, "OPR-10", "adjacent string literals have different suffixes");
                return bad(span);
            }
            suffix = Some(s);
        }
    }
    let Some(suffix) = suffix else { return bad(span) };
    let name = format!("{}{}", LITERAL, suffix);
    let found: Vec<EntityId> = crate::foundations::names::check::lookup(cx, &name).into_iter().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
    if found.is_empty() {
        // PRIM-26: a number runs over a sign after an exponent letter, so
        // the sign and what follows join the suffix.
        let over = suffix.contains('+') || suffix.contains('-');
        cx.error_cause(span, "PRIM-16", over.then_some("PRIM-26"), format!("no literal operator has the suffix '{}'", suffix));
        return bad(span);
    }
    let forms: Vec<(EntityId, Form)> = found.iter().filter_map(|&f| Some((f, literal_form(cx, &cx.u.func(f)?.ty.params)?))).collect();
    let pick = |want: &[Form]| want.iter().find_map(|w| forms.iter().find(|(_, g)| g == w).copied());
    let Some(Lit::UserDefined { base, raw, .. }) = &toks.iter().find_map(|t| matches!(t.lit, Some(Lit::UserDefined { .. })).then_some(&t.lit)).cloned().flatten() else { return bad(span) };
    // The raw form takes the characters of the literal without its
    // suffix, as a string.
    let raw_str = |text: &str| {
        let mut units: Vec<u32> = text.bytes().map(u32::from).collect();
        units.push(0);
        let el = QType { t: Type::Int(IntK::Char), c: true, v: false };
        let n = units.len() as u64;
        Expr::new(EK::Str(std::rc::Rc::new(units)), QType::new(Type::Array(std::rc::Rc::new(el), Some(n))), Cat::L, span)
    };
    let (chosen, args) = match &**base {
        Lit::Int { value, .. } => match pick(&[Form::Integer, Form::Raw]) {
            Some((f, Form::Integer)) => {
                if *value > u64::MAX as u128 {
                    cx.error(span, "PRIM-2", "integer literal too large for unsigned long long");
                    return bad(span);
                }
                (f, vec![Expr::int(*value as i128, QType::new(Type::Int(IntK::ULongLong)), span)])
            }
            Some((f, _)) => (f, vec![raw_str(raw)]),
            None => return no_form(cx, suffix, span),
        },
        Lit::Float { .. } => match pick(&[Form::Floating, Form::Raw]) {
            Some((f, Form::Floating)) => {
                let Some(x) = crate::foundations::types::run::F80::parse(raw) else {
                    let t = cx.unsupported(span, "a long double literal of this spelling");
                    return Expr::new(EK::Int(0), t, Cat::PR, span);
                };
                if !x.is_finite() {
                    cx.warning(span, "PRIM-13", "floating literal out of range");
                }
                (f, vec![Expr::new(EK::LongDouble(x), QType::new(Type::Float(FloatK::LongDouble)), Cat::PR, span)])
            }
            Some((f, _)) => (f, vec![raw_str(raw)]),
            None => return no_form(cx, suffix, span),
        },
        Lit::Char { prefix, value, multi } => {
            let k = crate::expressions::primary::check::char_kind(*prefix, *multi);
            match pick(&[Form::Char(k)]) {
                Some((f, _)) => (f, vec![Expr::int(*value, QType::new(Type::Int(k)), span)]),
                None => return no_form(cx, suffix, span),
            }
        }
        Lit::Str { .. } => {
            let plain: Vec<crate::lex::Token> = toks
                .iter()
                .map(|t| match &t.lit {
                    Some(Lit::UserDefined { base, .. }) => crate::lex::Token { lit: Some((**base).clone()), ..(*t).clone() },
                    _ => (*t).clone(),
                })
                .collect();
            let refs: Vec<&crate::lex::Token> = plain.iter().collect();
            let s = crate::expressions::primary::check::string(cx, &refs, span);
            let (k, n) = match &s.ty.t {
                Type::Array(el, Some(n)) => match el.t {
                    Type::Int(k) => (k, *n),
                    _ => return bad(span),
                },
                _ => return bad(span),
            };
            match pick(&[Form::Str(k)]) {
                Some((f, _)) => (f, vec![s, Expr::int(n as i128 - 1, QType::new(Type::Int(cx.u.target.size_t())), span)]),
                None => return no_form(cx, suffix, span),
            }
        }
        _ => return bad(span),
    };
    let callee = crate::expressions::primary::check::entity_expr(cx, &[chosen], &name, span);
    crate::expressions::postfix::check::finish_call(cx, callee, Some(chosen), args, span, None)
}

/// OPR-16: literal operators exist for the suffix, and none has the form
/// that the literal needs.
fn no_form(cx: &mut Check, suffix: &str, span: Span) -> Expr {
    cx.error(span, "OPR-16", format!("no literal operator for the suffix '{}' takes a literal of this kind", suffix));
    bad(span)
}
