//! `templates/concepts`, the Check segment: concept definitions,
//! concept-ids, requires expressions, and the satisfaction of a function
//! template's constraints (build step 5).

use crate::diag::Severity;
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::{QType, TArg};
use crate::sem::{EntKind, Linkage, ScopeKind};
use crate::syntax::{Kid, Node};

/// CPT-2, CPT-4: `template <...> concept C = constraint-expression;` at
/// namespace scope. False for another form.
pub fn concept_definition(cx: &mut Check, n: &Node) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() {
        cx.error(n.span, "CPT-4", "a concept defined outside namespace scope");
        return true;
    }
    let Some((params, defaults, constraints, pack)) = n.child("template_head").and_then(|h| crate::templates::declarations::check::type_params(cx, h)) else { return false };
    if defaults.iter().any(|d| d.is_some()) || !constraints.is_empty() || pack {
        return false;
    }
    let words = cx.words(n);
    let Some(name) = words.iter().skip_while(|w| **w != "concept").nth(1).map(|w| w.to_string()) else { return false };
    let Some(node) = n.kids.iter().rev().find_map(|k| match k {
        Kid::Node(c) if c.prod != "template_head" && c.prod != "attribute_spec" => Some(c.clone()),
        _ => None,
    }) else {
        return false;
    };
    let saved = cx.scope;
    if !names::declared_here(cx, saved, &name).is_empty() {
        return false;
    }
    if !definition_checks(cx, &node, &params) {
        return true;
    }
    // CPT-6: the definition does not use the concept's own name.
    let mut toks = Vec::new();
    node.tokens(&mut toks);
    if toks.iter().any(|&t| cx.toks[t].text == name) {
        cx.error(n.span, "CPT-6", format!("the definition of the concept '{}' uses its own name", name));
        return true;
    }
    let id = cx.u.concepts.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.concepts.push(crate::sem::Concept { name: name.as_str().into(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node, scope: saved, span: n.span, limit });
    names::declare_in(cx, saved, &name, EntKind::Concept(id), n.span, Linkage::External);
    true
}

/// The checks of a constraint expression at its definition: CPT-29 for
/// the parameters of its requires expressions, and CNS-39 for an atomic
/// constraint whose type is not `bool` whatever the arguments. False
/// after an error.
pub fn definition_checks(cx: &mut Check, n: &Node, params: &[crate::templates::declarations::check::Param]) -> bool {
    fn walk<'n>(n: &'n Node, prod: &str, out: &mut Vec<&'n Node>) {
        for k in &n.kids {
            if let Kid::Node(c) = k {
                if c.prod == prod {
                    out.push(c);
                }
                walk(c, prod, out);
            }
        }
    }
    let mut reqs = Vec::new();
    if n.prod == "requires_expression" {
        reqs.push(n);
    }
    walk(n, "requires_expression", &mut reqs);
    for r in reqs {
        let Some(ps) = r.child("parameters") else { continue };
        // CPT-29: no default argument, no `...`, no placeholder, and no
        // name of a template parameter.
        if cx.words(ps).contains(&"...") {
            cx.error(ps.span, "CPT-29", "a requires expression's parameters end with '...'");
            return false;
        }
        for p in ps.children("parameter") {
            let name = p.find("declarator_id").map(|d| cx.source(d));
            let why = if cx.words(p).contains(&"=") {
                Some("a default argument")
            } else if p.child("decl_specifiers").is_some_and(|ds| cx.words(ds).contains(&"auto") || ds.find("placeholder").is_some()) {
                Some("a placeholder type")
            } else if name.as_ref().is_some_and(|nm| params.iter().any(|q| q.0.as_ref() == nm.as_str())) {
                Some("the name of a template parameter")
            } else {
                None
            };
            if let Some(why) = why {
                cx.error(p.span, "CPT-29", format!("a parameter of a requires expression with {}", why));
                return false;
            }
        }
    }
    // CNS-39: `sizeof`, `alignof`, or an integer literal as an atomic
    // constraint has a type other than `bool` whatever the arguments.
    fn atomics<'n>(n: &'n Node, cx: &Check, out: &mut Vec<&'n Node>) {
        match n.prod {
            "constraint_or" | "logical_or_expr" | "constraint_and" | "logical_and_expr" | "requires_clause" => {
                for k in &n.kids {
                    if let Kid::Node(c) = k {
                        atomics(c, cx, out);
                    }
                }
            }
            "primary" if n.kids.len() == 3 && cx.first_word(n) == "(" => {
                if let Some(Kid::Node(c)) = n.kids.get(1) {
                    atomics(c, cx, out);
                }
            }
            _ => out.push(n),
        }
    }
    let mut ats = Vec::new();
    atomics(n, cx, &mut ats);
    for a in ats {
        let not_bool = (a.prod == "unary_expr" && matches!(cx.first_word(a), "sizeof" | "alignof")) || (a.prod == "literal" && cx.first_word(a).chars().next().is_some_and(|c| c.is_ascii_digit()));
        if not_bool {
            cx.error(a.span, "CNS-39", "an atomic constraint whose type is not bool");
            return false;
        }
    }
    true
}

/// CPT-1: `C<A...>`, a concept-id: a prvalue of type `bool` that tells
/// whether the arguments satisfy the concept. None when the name is no
/// concept.
pub fn concept_use(cx: &mut Check, tid: &Node) -> Option<Expr> {
    let span = tid.span;
    let name = cx.source(tid.child("template_name")?);
    let c = names::lookup(cx, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Concept(c) => Some(c),
        _ => None,
    })?;
    let kinds = cx.u.concepts[c].kinds.clone();
    let marks = cx.u.diags.len();
    let Some(args) = crate::templates::declarations::check::template_args(cx, tid, &kinds) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == Severity::Error) {
            return Some(crate::expressions::check::bad(span));
        }
        let t = cx.unsupported(span, "a concept-id with arguments of another number or form");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(crate::expressions::check::bad(span));
    }
    Some(match satisfied(cx, c, &args, span) {
        Some(v) => Expr::int(v as i128, QType::bool(), span),
        None => {
            let t = cx.unsupported(span, "a concept-id whose satisfaction the first pieces of build step 5 do not check");
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
    })
}

/// CNS-5 to CNS-9: whether the arguments `args` satisfy the concept `c`.
/// None when the check needs a part of the model that is not there.
pub fn satisfied(cx: &mut Check, c: usize, args: &[TArg], span: crate::source::Span) -> Option<bool> {
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        return None;
    }
    let ct = cx.u.concepts[c].clone();
    if args.len() != ct.params.len() {
        return None;
    }
    // CNS-10: a satisfaction keeps its first result.
    if let Some(&(_, _, v)) = cx.satisfactions.iter().find(|(k, a, _)| *k == c && a.as_slice() == args) {
        return Some(v);
    }
    // CNS-35: the check does not need its own result. The check that
    // began the loop reports it.
    if cx.satisfying.iter().any(|(k, a)| *k == c && a.as_slice() == args) {
        cx.satisfaction_loop = true;
        return Some(false);
    }
    cx.satisfying.push((c, args.to_vec()));
    // CNS-7: argument-dependent lookup in a concept's constraint is at
    // the point of the check.
    let names_ = ct.params.clone();
    let saved = std::mem::replace(&mut cx.constraint_names, names_);
    let saved_adl = std::mem::replace(&mut cx.adl_at_check, true);
    // Access in the requirements is checked where the concept-id is, as
    // the matrix compilers do (Q-0288): the function stays.
    let func = cx.func.clone();
    let r = within(cx, ct.scope, ct.limit, &ct.params, args, span, |cx| {
        cx.func = func;
        let r = constraint(cx, &ct.node);
        cx.func = None;
        r
    });
    cx.adl_at_check = saved_adl;
    if let Some(v) = r {
        cx.satisfactions.push((c, args.to_vec(), v));
    }
    cx.constraint_names = saved;
    cx.satisfying.pop();
    if cx.satisfying.is_empty() && std::mem::take(&mut cx.satisfaction_loop) {
        cx.error(span, "CNS-35", format!("the satisfaction of '{}' depends on itself", ct.name));
        return Some(false);
    }
    r
}

/// Run `f` in a scope of the template's scope `scope` with the
/// parameters `names_` bound to `args`.
pub fn within<T>(cx: &mut Check, scope: crate::sem::ScopeId, limit: crate::sem::EntityId, names_: &[std::rc::Rc<str>], args: &[TArg], span: crate::source::Span, f: impl FnOnce(&mut Check) -> T) -> T {
    let saved_scope = cx.scope;
    let saved_limit = cx.lookup_limit.replace(limit);
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    cx.scope = scope;
    let ts = cx.enter(ScopeKind::Template);
    crate::templates::declarations::check::bind_params(cx, ts, names_, args, span);
    let r = f(cx);
    cx.scope = saved_scope;
    cx.lookup_limit = saved_limit;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    r
}

/// Check `f` quietly: Some(true) when it gives no error, Some(false) when
/// it gives one (a substitution failure, CNS-7), and None when it meets a
/// construct that the front end does not model. The errors are dropped.
fn quietly(cx: &mut Check, f: impl FnOnce(&mut Check)) -> Option<bool> {
    let marks = cx.u.diags.len();
    cx.instantiating += 1;
    f(cx);
    cx.instantiating -= 1;
    let tail = cx.u.diags.split_off(marks);
    if tail.iter().any(|d| d.sev == Severity::Unsupported) {
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev == Severity::Unsupported));
        return None;
    }
    // A warning that -Werror makes an error is no substitution failure.
    Some(!tail.iter().any(|d| d.sev == Severity::Error && !d.msg.ends_with("(warning treated as error)")))
}

/// CNS-2 to CNS-9: whether a constraint expression is satisfied: a
/// conjunction or a disjunction of constraints, checked left to right,
/// or an atomic constraint, a constant expression of type `bool` whose
/// substitution can fail.
pub fn constraint(cx: &mut Check, n: &Node) -> Option<bool> {
    let parts: Vec<&Node> = n.kids.iter().filter_map(|k| match k {
        Kid::Node(c) => Some(c),
        _ => None,
    }).collect();
    match n.prod {
        "constraint_or" | "logical_or_expr" | "requires_clause" => {
            for (k, p) in parts.iter().enumerate() {
                if constraint(cx, p)? {
                    return skipped_bool(cx, &parts[k + 1..]).then_some(true);
                }
            }
            Some(false)
        }
        "constraint_and" | "logical_and_expr" => {
            for (k, p) in parts.iter().enumerate() {
                if !constraint(cx, p)? {
                    return skipped_bool(cx, &parts[k + 1..]).then_some(false);
                }
            }
            Some(true)
        }
        // `( E )`: the constraint inside.
        "primary" if n.kids.len() == 3 && parts.len() == 1 && cx.first_word(n) == "(" => constraint(cx, parts[0]),
        "primary" if parts.len() == 1 && parts[0].prod == "requires_expression" => requirements(cx, parts[0]),
        _ => {
            // CNS-7: an atomic constraint whose substitution fails is not
            // satisfied.
            let mut e = None;
            quietly(cx, |cx| e = Some(crate::expressions::check::expr(cx, n))).and_then(|ok| {
                if !ok {
                    return Some(false);
                }
                let e = crate::foundations::values::check::rvalue(cx, e?);
                // CNS-9: the expression has type `bool`.
                if !e.ty.unqual().is_bool() {
                    cx.error(n.span, "CNS-9", format!("an atomic constraint of type {}, not bool", e.ty.show()));
                    return Some(false);
                }
                let v = crate::foundations::constant_evaluation::check::try_value(cx, &e).and_then(|v| v.int());
                if v.is_none() {
                    cx.error(n.span, "CNS-9", "an atomic constraint that is not a constant expression");
                }
                Some(v? != 0)
            })
        }
    }
}

/// CNS-39: an atomic constraint that the check skips has type `bool`
/// when its type depends on no parameter. The first pieces of build step
/// 5 do not tell, so a skipped constraint of another type gives false,
/// which leaves the check to an unsupported answer.
fn skipped_bool(cx: &mut Check, rest: &[&Node]) -> bool {
    rest.iter().all(|n| match n.prod {
        "constraint_or" | "logical_or_expr" | "constraint_and" | "logical_and_expr" => skipped_bool(cx, &n.kids.iter().filter_map(|k| match k {
            Kid::Node(c) => Some(c),
            _ => None,
        }).collect::<Vec<_>>()),
        "primary" if n.kids.len() == 3 && cx.first_word(n) == "(" => skipped_bool(cx, &n.kids.iter().filter_map(|k| match k {
            Kid::Node(c) => Some(c),
            _ => None,
        }).collect::<Vec<_>>()),
        "primary" if n.child("requires_expression").is_some() => true,
        _ => {
            let mut ty = None;
            let ok = quietly(cx, |cx| {
                cx.unevaluated += 1;
                ty = Some(crate::expressions::check::expr(cx, n).ty);
                cx.unevaluated -= 1;
            });
            ok != Some(true) || ty.is_some_and(|t| t.nonref().unqual().is_bool())
        }
    })
}

/// CPT-24: `requires (P...) { R... }` as an expression outside a
/// constraint. A requirement there that depends on no template parameter
/// must be valid (CPT-27), which the first pieces of build step 5 do not
/// tell from one that does, so the expression stays unsupported.
pub fn requires_expression(cx: &mut Check, n: &Node) -> Expr {
    let t = cx.unsupported(n.span, "a requires expression outside a constraint (build step 5)");
    Expr::new(EK::Int(0), t, Cat::PR, n.span)
}

/// CPT-25 to CPT-35: whether the requirements of a requires expression
/// hold, checked in order with its parameters in scope.
fn requirements(cx: &mut Check, n: &Node) -> Option<bool> {
    let saved = cx.scope;
    cx.enter(ScopeKind::Block);
    let r = (|| {
        // CPT-28: the parameters are names with types.
        if let Some(ps) = n.child("parameters") {
            for p in ps.children("parameter") {
                let ok = quietly(cx, |cx| {
                    let Some(ds) = p.child("decl_specifiers") else { return };
                    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
                    let Some(dn) = p.child("declarator") else { return };
                    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
                    let Some((name, span)) = d.name.clone() else { return };
                    // CPT-28: the types adjust as a function's parameters
                    // do.
                    let ty = crate::declarations::auto::check::decay(&d.ty);
                    let var = crate::sem::Var { ty, storage: crate::sem::Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: true, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
                    names::declare_in(cx, cx.scope, &name, EntKind::Var(var), span, Linkage::None);
                })?;
                if !ok {
                    return Some(false);
                }
            }
        }
        for r in n.children("requirement") {
            let Some(Kid::Node(k)) = r.kids.first() else { return None };
            // A destructor call's validity needs the eligibility of
            // destructors (CNS-21), which waits for a later piece.
            if cx.words(r).contains(&"~") || { let mut t = Vec::new(); r.tokens(&mut t); t.iter().any(|&x| cx.toks[x].text == "~") } {
                return None;
            }
            let holds = match k.prod {
                // CPT-31: the expression is valid.
                "simple_requirement" => {
                    let e = k.kids.iter().find_map(|x| match x {
                        Kid::Node(c) => Some(c),
                        _ => None,
                    })?;
                    unevaluated(cx, |cx| {
                        crate::expressions::check::expr(cx, e);
                    })?
                }
                // CPT-32: the name forms a type.
                "type_requirement" => unevaluated(cx, |cx| {
                    crate::templates::dependent_names::check::typename_specifier(cx, k);
                })?,
                // CPT-33: the expression is valid, and its type satisfies
                // the type constraint.
                "compound_requirement" => {
                    if cx.has_word(k, "noexcept") {
                        return None;
                    }
                    let e = k.kids.iter().find_map(|x| match x {
                        Kid::Node(c) if c.prod != "type_constraint" => Some(c),
                        _ => None,
                    })?;
                    let mut ty = None;
                    if !unevaluated(cx, |cx| {
                        let x = crate::expressions::check::expr(cx, e);
                        ty = Some(crate::expressions::decltype::check::decltype_of(&x));
                    })? {
                        false
                    } else {
                        match k.child("type_constraint") {
                            Some(tc) => type_constraint(cx, tc, ty?)?,
                            None => true,
                        }
                    }
                }
                // CPT-35: the constraint is satisfied.
                "nested_requirement" => {
                    let c = k.kids.iter().find_map(|x| match x {
                        Kid::Node(c) => Some(c),
                        _ => None,
                    })?;
                    constraint(cx, c)?
                }
                _ => return None,
            };
            if !holds {
                // CPT-27: a requirement that names no parameter must be
                // valid; the first pieces of build step 5 leave it.
                let mut toks = Vec::new();
                r.tokens(&mut toks);
                let req_params: Vec<String> = n.child("parameters").map(|ps| ps.children("parameter").filter_map(|p| p.find("declarator_id").map(|d| cx.source(d))).collect()).unwrap_or_default();
                if !toks.iter().any(|&t| cx.constraint_names.iter().any(|p| p.as_ref() == cx.toks[t].text) || req_params.iter().any(|p| *p == cx.toks[t].text)) {
                    return None;
                }
                return Some(false);
            }
        }
        Some(true)
    })();
    cx.leave(saved);
    r
}

/// `quietly` in an unevaluated operand (CPT-30).
fn unevaluated(cx: &mut Check, f: impl FnOnce(&mut Check)) -> Option<bool> {
    cx.unevaluated += 1;
    let r = quietly(cx, f);
    cx.unevaluated -= 1;
    r
}

/// CPT-17, CPT-33: whether the type `t` satisfies the type constraint
/// `C` or `C<A...>`: the concept with `t` first and the arguments after.
pub fn type_constraint(cx: &mut Check, tc: &Node, t: QType) -> Option<bool> {
    if tc.child("nested_name").is_some() {
        return None;
    }
    let name = cx.words(tc).first()?.to_string();
    let c = names::lookup(cx, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Concept(c) => Some(c),
        _ => None,
    })?;
    let kinds = cx.u.concepts[c].kinds.clone();
    if kinds.first().is_some_and(|k| k.is_some()) {
        return None;
    }
    let mut args = vec![TArg::Type(t)];
    if tc.child("template_args").is_some() {
        // CNS-7: an argument whose substitution fails is not satisfied.
        let mut more = None;
        if !quietly(cx, |cx| more = crate::templates::declarations::check::template_args(cx, tc, &kinds[1..]))? {
            return Some(false);
        }
        args.extend(more?);
    } else if kinds.len() != 1 {
        return None;
    }
    satisfied(cx, c, &args, tc.span)
}

/// CNS-11: whether the arguments `args` of the function template `t`
/// satisfy its constraints. None when the check needs a part of the
/// model that is not there.
pub fn function_satisfied(cx: &mut Check, t: usize, args: &[TArg]) -> Option<bool> {
    let tpl = cx.u.templates[t].clone();
    if tpl.constraints.is_empty() {
        return Some(true);
    }
    let saved = std::mem::replace(&mut cx.constraint_names, tpl.params.clone());
    let r = within(cx, tpl.scope, tpl.limit, &tpl.params, args, tpl.span, |cx| {
        for (k, node) in &tpl.constraints {
            let ok = match k {
                None => constraint(cx, node)?,
                Some(k) => match args.get(*k)? {
                    TArg::Type(t) => type_constraint(cx, node, t.clone())?,
                    TArg::Value(..) | TArg::Pack(_) => return None,
                },
            };
            if !ok {
                return Some(false);
            }
        }
        Some(true)
    });
    cx.constraint_names = saved;
    r
}

/// CNS-11: whether a member function's trailing requires clause is
/// satisfied, checked in the scope of its class specialization. True for
/// a function without one.
pub fn member_satisfied(cx: &mut Check, f: crate::sem::EntityId) -> Option<bool> {
    let Some((node, scope)) = cx.u.member_constraints.get(&f).cloned() else { return Some(true) };
    let saved_scope = std::mem::replace(&mut cx.scope, scope);
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let r = constraint(cx, &node);
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    r
}
