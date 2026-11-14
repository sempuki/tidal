//! The check part of `classes/operators`, for build step 4 part a:
//! operator functions that are members or non-members and not templates
//! (OPR-1, OPR-2), and the selection of an operator function for an
//! expression with an operand of class or enumeration type (OPR-6,
//! OPR-15). Literal operators, `operator->`, the allocation functions,
//! and the rewritten candidates of `<=>` are unsupported.

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
    // The allocation and deallocation functions come with new and
    // delete (build step 4).
    if spelling.starts_with("new") || spelling.starts_with("delete") {
        return None;
    }
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
pub fn declaration(cx: &mut Check, name: &str, ty: &QType, defaults: bool, class: Option<crate::sem::types::ClassId>, is_static: bool, span: Span) -> bool {
    let op = &name["operator".len()..];
    if matches!(op, "new" | "delete" | "new[]" | "delete[]") {
        cx.unsupported(span, "an allocation or deallocation function (build step 4)");
        return false;
    }
    if matches!(op, "co_await" | "->") {
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
    let count = f.params.len() + usize::from(class.is_some() && !is_static);
    let ok = match op {
        "!" | "~" => count == 1,
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
    let rest = if c.member && !func.member_static {
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
            None if !(c.member && k == 0 && func.member_static) => out.push(user_conversion(cx, a, p)?),
            None => return None,
        }
    }
    Some(out)
}

/// A user-defined conversion sequence from `a` to a parameter of class
/// type, or a reference to const of it: a converting constructor that
/// takes `a` by a standard sequence.
fn user_conversion(cx: &Check, a: &Expr, p: &QType) -> Option<Ics> {
    if p.is_ref() && !(p.nonref().c && !p.nonref().v) && !matches!(p.t, Type::Ref(_, true)) {
        return None;
    }
    let c = p.nonref().class()?;
    if a.ty.nonref().class() == Some(c) || a.ty.nonref().is_class() {
        return None;
    }
    let ok = crate::classes::constructors::check::ctors(cx, c).into_iter().any(|f| {
        cx.u.func(f).is_some_and(|x| !x.explicit && !x.deleted && x.ty.params.len() == 1 && sequence(cx, a, &x.ty.params[0]).is_some())
    });
    ok.then(|| Ics { rank: crate::overloading::conversion_sequences::check::Rank::User, qual: false, binds: None, ellipsis: false })
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
        some || (!(a.0.reversed || a.0.negated) && (b.0.reversed || b.0.negated)) || (!a.0.reversed && b.0.reversed)
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
        let obj = if func.member_static { None } else { Some(obj) };
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

fn user_type(t: &QType) -> bool {
    let t = t.nonref();
    t.is_class() || t.is_enum()
}

/// A binary operator `op` with an operand of class or enumeration type
/// (OPR-6): the call of the operator function that overload resolution
/// chooses. None when the built-in operator applies: no operand of such
/// a type, or no operator function for an enumeration operand.
pub fn binary(cx: &mut Check, op: &str, l: &Expr, r: &Expr, span: Span) -> Option<Expr> {
    if l.ty.is_error() || r.ty.is_error() || !(user_type(&l.ty) || user_type(&r.ty)) {
        return None;
    }
    let name = name_of(op)?;
    let args = vec![l.clone(), r.clone()];
    let (members, others) = found(cx, name, &args, Some(l));
    let mut cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false }).chain(others.iter().map(|&f| Cand { f, member: false, reversed: false, negated: false })).collect();
    // The rewritten candidates of `==` and `!=`: `y == x`, and the
    // opposite of `x == y` and `y == x`.
    let mut excluded = false;
    if matches!(op, "==" | "!=") {
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
            cands.extend(m1.iter().map(|&f| Cand { f, member: true, reversed: false, negated: true }));
            cands.extend(o1.iter().map(|&f| Cand { f, member: false, reversed: false, negated: true }));
        }
        cands.extend(m2.iter().map(|&f| Cand { f, member: true, reversed: true, negated }));
        cands.extend(o1.iter().map(|&f| Cand { f, member: false, reversed: true, negated }));
    }
    if excluded && !cands.iter().any(|c| {
        let ordered: Vec<Expr> = if c.reversed { args.iter().rev().cloned().collect() } else { args.clone() };
        sequences(cx, c, &ordered).is_some()
    }) {
        cx.error_cause(span, "DCMP-12", Some("DCMP-13"), format!("the operator '{}' needs an operator== that a matching operator!= keeps from rewriting", op));
        return Some(bad(span));
    }
    // The reversed candidates of `<=>` are unsupported.
    if op == "<=>" {
        let (m2, o2) = found(cx, "operator<=>", &args, Some(r));
        let reversed = m2.iter().chain(o2.iter()).any(|&f| {
            let c = Cand { f, member: m2.contains(&f), reversed: true, negated: false };
            let ordered: Vec<Expr> = args.iter().rev().cloned().collect();
            sequences(cx, &c, &ordered).is_some()
        });
        if reversed {
            let t = cx.unsupported(span, "a three-way comparison with a reversed candidate");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
    }
    if matches!(op, "<" | ">" | "<=" | ">=" | "==" | "!=") {
        let (m, o) = found(cx, "operator<=>", &args, Some(l));
        let (m2, _) = found(cx, "operator<=>", &args, Some(r));
        if !(m.is_empty() && o.is_empty() && m2.is_empty()) && !matches!(op, "==" | "!=") {
            let t = cx.unsupported(span, "a comparison that a three-way comparison operator rewrites");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
    }
    // A defaulted comparison is unsupported.
    if cands.iter().any(|c| cx.u.func(c.f).is_some_and(|f| f.unsupported.is_some())) {
        let t = cx.unsupported(span, "an operator function that the front end does not model");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    }
    if cands.is_empty() {
        return None;
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
    ne.into_iter().any(|g| {
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
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false }).chain(others.iter().map(|&f| Cand { f, member: false, reversed: false, negated: false })).collect();
    if cands.is_empty() {
        return None;
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
    if !e.ty.nonref().is_class() {
        let t = cx.unsupported(span, "an operator function against a built-in operator for an enumeration operand");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
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
        // The implicit or defaulted assignment: each member in turn where
        // a member runs code (CPY-8).
        if r.ty.nonref().class() == Some(c) && needs_assign(cx, c) && !l.ty.c {
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
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false }).collect();
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
    cx.u.field_types(c).iter().any(|t| {
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
    let mut parts: Vec<Expr> = Vec::new();
    for f in cx.u.classes[c].fields.clone() {
        let lm = crate::classes::members::check::field_of(cx, l.clone(), f, span);
        let rm = crate::classes::members::check::field_of(cx, r.clone(), f, span);
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
    let cands: Vec<Cand> = members.iter().map(|&f| Cand { f, member: true, reversed: false, negated: false }).collect();
    let mut all = vec![a.clone()];
    all.extend(args.iter().cloned());
    let c = choose(cx, &cands, &all, span, "[]")?;
    Some(build(cx, c, all, span))
}

/// The class has an `operator->` (OPR-5), which is unsupported.
pub fn has_arrow(cx: &Check, t: &QType) -> bool {
    t.nonref().class().is_some_and(|c| !crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator->").is_empty())
}

/// The operator functions named by an entity, for diagnostics.
pub fn is_operator_entity(cx: &Check, e: EntityId) -> bool {
    matches!(cx.u.entities[e].kind, EntKind::Func(_)) && is_operator(&cx.u.entities[e].name)
}
