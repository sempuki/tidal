//! The check part of `templates/declarations`: function templates whose
//! parameters are types (build step 5, first piece,
//! `frontend/TEMPLATE-MODEL.md` sections 1 and 3). The declaration gives
//! the function type with each parameter as a dependent type. A call
//! deduces the arguments (`templates/deduction`), and the specialization
//! is the definition checked with each parameter bound to its argument.
//! The definition itself is not checked yet (model section 5), so a
//! template that nothing specializes leaves the unit unsupported.

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::{DepType, QType, TArg, Type};
use crate::sem::{EntKind, EntityId, FnTemplate, Linkage, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// A template parameter: its name, where it is declared, and None for a
/// type parameter or the type of a constant parameter.
type Param = (Rc<str>, Span, Option<QType>);

/// The parameters of a template head whose parameters are type
/// parameters and constant parameters of integral or enumeration type,
/// without a pack or a constraint, with the default of each (TPL-13);
/// None for another head.
fn type_params(cx: &mut Check, head: &Node) -> Option<(Vec<Param>, Vec<Option<Node>>)> {
    if head.child("requires_clause").is_some() {
        return None;
    }
    let mut out = Vec::new();
    let mut defaults = Vec::new();
    for tp in head.child("template_params")?.children("template_param") {
        // TARG-13: a constant parameter of integral or enumeration type.
        if let Some(p) = tp.child("constant_param").and_then(|c| c.child("parameter")) {
            if cx.words(p).contains(&"...") || p.find("ptr_part").is_some() {
                return None;
            }
            // The default is the expression after `=`.
            let default = match cx.words(p).contains(&"=") {
                true => Some(p.kids.iter().rev().find_map(|k| match k {
                    Kid::Node(c) => Some(c.clone()),
                    _ => None,
                })?),
                false => None,
            };
            let ds = p.child("decl_specifiers")?;
            let marks = cx.u.diags.len();
            let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
            let ty = specs.ty.clone().filter(|t| (t.is_integral() || t.is_enum()) && cx.u.diags.len() == marks && !specs.unsupported);
            let Some(ty) = ty else {
                cx.u.diags.truncate(marks);
                return None;
            };
            let id = p.find("declarator_id")?;
            out.push((cx.source(id).as_str().into(), id.span, Some(ty.unqual())));
            defaults.push(default);
            continue;
        }
        let t = tp.child("type_param")?;
        if t.child("type_key").is_none() || t.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && cx.text(k) == "...") {
            return None;
        }
        let k = t.kids.iter().rev().find(|k| matches!(k, Kid::Tok(_)) && cx.text(k) != "=")?;
        let name: Rc<str> = cx.text(k).into();
        if name.as_ref() == "class" || name.as_ref() == "typename" {
            return None;
        }
        out.push((name, cx.token(k).map(|t| t.span).unwrap_or(t.span), None));
        defaults.push(t.child("type_id").cloned());
    }
    Some((out, defaults))
}

/// TPL-14: unless the template is a function template, each parameter
/// after one with a default has a default.
fn defaults_last(cx: &mut Check, defaults: &[Option<Node>], span: Span) -> bool {
    if let Some(k) = defaults.iter().position(|d| d.is_some()) {
        if defaults[k..].iter().any(|d| d.is_none()) {
            cx.error(span, "TPL-14", "a template parameter without a default after one with a default");
            return false;
        }
    }
    true
}

/// TPL-15, TPL-16: the defaults of a later declaration add to those of an
/// earlier one, and the two do not both give one parameter a default.
fn merge_defaults(cx: &mut Check, earlier: &[Option<Node>], later: &[Option<Node>], span: Span) -> Option<Vec<Option<Node>>> {
    if earlier.iter().zip(later).any(|(a, b)| a.is_some() && b.is_some()) {
        cx.error(span, "TPL-16", "a default template argument given again in a later declaration");
        return None;
    }
    Some(earlier.iter().zip(later).map(|(a, b)| a.clone().or_else(|| b.clone())).collect())
}

/// The arguments of a template-id, with the defaults of the template for
/// the parameters after them (TPL-13). Each default sees the parameters
/// before it bound to their arguments.
pub fn args_with_defaults(cx: &mut Check, tid: &Node, kinds: &[Option<QType>], defaults: &[Option<Node>], names_: &[Rc<str>], scope: crate::sem::ScopeId, limit: EntityId) -> Option<Vec<TArg>> {
    let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
    if n >= kinds.len() {
        return template_args(cx, tid, kinds);
    }
    let mut args = template_args(cx, tid, &kinds[..n])?;
    fill_defaults(cx, kinds, defaults, names_, scope, limit, &mut args, tid.span).then_some(args)
}

/// Fill `args` up to the parameters' number from their defaults.
fn fill_defaults(cx: &mut Check, kinds: &[Option<QType>], defaults: &[Option<Node>], names_: &[Rc<str>], scope: crate::sem::ScopeId, limit: EntityId, args: &mut Vec<TArg>, span: Span) -> bool {
    while args.len() < kinds.len() {
        let k = args.len();
        let Some(Some(d)) = defaults.get(k).cloned() else { return false };
        let saved_scope = cx.scope;
        let saved_limit = cx.lookup_limit.replace(limit);
        cx.scope = scope;
        let ts = cx.enter(ScopeKind::Template);
        bind_params(cx, ts, &names_[..k], args, span);
        let a = match &kinds[k] {
            None => Some(TArg::Type(crate::declarations::declarators::check::type_id(cx, &d))),
            Some(ty) => {
                let e = crate::expressions::check::expr(cx, &d);
                let e = crate::foundations::values::check::rvalue(cx, e);
                let v = (e.ty.is_integral() || e.ty.is_enum()).then(|| crate::foundations::constant_evaluation::check::try_value(cx, &e).and_then(|v| v.int())).flatten();
                v.and_then(|v| constant_of(cx, &e, v, ty, d.span))
            }
        };
        cx.scope = saved_scope;
        cx.lookup_limit = saved_limit;
        match a {
            Some(a) => args.push(a),
            None => return false,
        }
    }
    true
}

/// Declare the parameters in the scope `ts` at a definition: a type
/// parameter as a dependent type, a constant parameter as a variable of
/// its type whose value is not known.
fn declare_params(cx: &mut Check, ts: crate::sem::ScopeId, params: &[Param]) {
    for (k, (name, span, kind)) in params.iter().enumerate() {
        match kind {
            None => {
                let t = QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: name.clone() })));
                names::declare_in(cx, ts, name, EntKind::Typedef(t), *span, Linkage::None);
            }
            Some(ty) => {
                let v = crate::sem::Var { ty: ty.with_cv(true, false), storage: crate::sem::Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: true, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
                names::declare_in(cx, ts, name, EntKind::Var(v), *span, Linkage::None);
            }
        }
    }
}

/// Bind the parameters `names` to the arguments `args` in the scope `ts`
/// of a specialization: a type parameter names its type, and a constant
/// parameter is a constant of its value (TARG-13).
fn bind_params(cx: &mut Check, ts: crate::sem::ScopeId, names_: &[Rc<str>], args: &[TArg], span: Span) {
    for (name, a) in names_.iter().zip(args) {
        match a {
            TArg::Type(t) => {
                names::declare_in(cx, ts, name, EntKind::Typedef(t.clone()), span, Linkage::None);
            }
            TArg::Value(t, v) => {
                let init = crate::sem::tree::Init::Expr(crate::sem::tree::Expr::int(*v, t.clone(), span));
                let var = crate::sem::Var { ty: t.with_cv(true, false), storage: crate::sem::Storage::Static, constexpr: true, constinit: false, inline: true, is_extern: false, is_static: false, param: false, defined: true, init: Some(init), const_init: true, used: true, maybe_unused: true, align: None };
                let id = names::declare_in(cx, ts, name, EntKind::Var(var), span, Linkage::None);
                cx.template_constants.insert(id);
            }
        }
    }
}

/// The arguments of a template-id for parameters of the kinds `kinds`: a
/// type for a type parameter, and a constant for a constant parameter,
/// converted to its type. None for another number or form of arguments.
pub fn template_args(cx: &mut Check, tid: &Node, kinds: &[Option<QType>]) -> Option<Vec<TArg>> {
    let nodes: Vec<&Node> = tid.child("template_args").map(|ta| ta.children("template_arg").collect()).unwrap_or_default();
    if nodes.len() != kinds.len() {
        return None;
    }
    let mut out = Vec::new();
    for (a, kind) in nodes.into_iter().zip(kinds) {
        match kind {
            None => out.push(TArg::Type(crate::declarations::declarators::check::type_id(cx, a.child("type_id")?))),
            Some(ty) => {
                let (e, v) = constant_arg(cx, a)?;
                out.push(constant_of(cx, &e, v, ty, a.span)?);
            }
        }
    }
    Some(out)
}

/// TARG-13, CE-19: the constant `e` of value `v` as an argument for a
/// constant parameter of type `ty`. It converts as a converted constant
/// expression: an enumeration only from itself, and no narrowing.
fn constant_of(cx: &mut Check, e: &crate::sem::tree::Expr, v: i128, ty: &QType, span: Span) -> Option<TArg> {
    let scoped = matches!(ty.t, Type::Enum(en) if cx.u.enums[en].scoped);
    let other_enum = (ty.is_enum() || matches!(e.ty.t, Type::Enum(en) if cx.u.enums[en].scoped)) && e.ty.unqual().t != ty.t;
    if (scoped || other_enum) && e.ty.unqual().t != ty.t || crate::foundations::conversions::check::is_narrowing(cx, e, ty) {
        cx.error_cause(span, "TARG-13", Some("CE-19"), format!("the template argument of type {} does not convert to {} as a constant", e.ty.show(), ty.show()));
        return None;
    }
    let v = match ty.int_kind() {
        Some(k) => cx.u.target.wrap(k, v),
        None => v,
    };
    Some(TArg::Value(ty.clone(), v))
}

/// The value of a template argument for a constant parameter: an
/// expression, or a name that the parser read as a type.
fn constant_arg(cx: &mut Check, a: &Node) -> Option<(crate::sem::tree::Expr, i128)> {
    let node = crate::expressions::check::first_node(a)?;
    let e = if node.prod == "type_id" {
        let name = cx.source(node);
        let var = names::lookup(cx, &name).into_iter().find(|&e| cx.u.var(e).is_some())?;
        let ty = cx.u.var(var)?.ty.clone();
        crate::sem::tree::Expr::new(crate::sem::tree::EK::Var(var), ty.nonref(), crate::sem::tree::Cat::L, node.span)
    } else {
        crate::expressions::check::expr(cx, node)
    };
    let e = crate::foundations::values::check::rvalue(cx, e);
    if e.ty.is_error() || !(e.ty.is_integral() || e.ty.is_enum()) {
        return None;
    }
    let v = crate::foundations::constant_evaluation::check::try_value(cx, &e).and_then(|v| v.int())?;
    Some((e, v))
}

/// `template_decl` at namespace scope: a function template definition
/// whose parameters are types. False for another form, which the caller
/// reports as unsupported.
pub fn template_decl(cx: &mut Check, n: &Node) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() {
        return false;
    }
    if c_linkage(cx, n) {
        return true;
    }
    let Some((params, defaults)) = n.child("template_head").and_then(|h| type_params(cx, h)) else { return false };
    if let Some(done) = outside_member(cx, n, &params) {
        // TPL-17: a member defined after the class body gives no default.
        if done && defaults.iter().any(|d| d.is_some()) {
            cx.error(n.span, "TPL-17", "a default template argument on a member defined outside its class template");
        }
        return done;
    }
    if let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) {
        if sd.child("init_declarator").is_none() {
            return class_template_decl(cx, sd, params, defaults);
        }
        // `template <class T> R f(P);` declares a function template.
        let mut ids = sd.children("init_declarator");
        let one = ids.next().filter(|i| i.child("init").is_none() && i.child("declarator").is_some_and(|d| d.find("function_suffix").is_some()));
        if let (Some(one), None) = (one, ids.next()) {
            let (Some(ds), Some(dn)) = (sd.child("decl_specifiers"), one.child("declarator")) else { return false };
            return function_template(cx, sd, ds, dn, params, defaults);
        }
        return var_template_decl(cx, sd, params, defaults);
    }
    let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) else { return false };
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    function_template(cx, fd, ds, dn, params, defaults)
}

/// TPL rules: a function template's declaration `fd`, a definition or
/// not, with its decl-specifiers `ds` and its declarator `dn`. A
/// definition after a declaration completes the template.
fn function_template(cx: &mut Check, fd: &Node, ds: &Node, dn: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>) -> bool {
    if shadows_param(cx, fd, &params) || expands_no_pack(cx, fd) {
        return true;
    }
    let saved = cx.scope;
    let ts = cx.enter(ScopeKind::Template);
    declare_params(cx, ts, &params);
    let marks = cx.u.diags.len();
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.leave(saved);
    let fty = d.ty.func().cloned();
    let (Some((name, span)), Some(fty)) = (d.name.clone(), fty) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    if specs.unsupported || d.unsupported || d.qual != names::Qual::None || cx.u.diags.len() > marks || d.placeholder.is_some() {
        cx.u.diags.truncate(marks);
        return false;
    }
    // TPL-29: `main` is no template.
    if name.as_ref() == "main" && saved == cx.u.root {
        cx.error(span, "TPL-29", "the function 'main' declared as a template");
        return true;
    }
    // The definition checks of model section 5 wait for a later piece, so
    // a definition with a dependent construct that the first piece does
    // not model, or a name another function or template shares, stays
    // unsupported.
    if dependent_forms(cx, fd, &params) {
        return false;
    }
    let found = names::declared_here(cx, saved, &name);
    if let [e] = found[..] {
        // A second declaration of the template, which a definition
        // completes; a template of the name with another type is an
        // overload that waits for a later piece.
        let EntKind::Template(t) = cx.u.entities[e].kind else { return false };
        let tpl = &cx.u.templates[t];
        let kinds: Vec<Option<QType>> = params.iter().map(|p| p.2.clone()).collect();
        if *tpl.fty != fty || tpl.kinds != kinds {
            return false;
        }
        // DEFARG-13: a later declaration adds no default argument.
        if let Some(p) = dn.find("parameters").and_then(|ps| ps.children("parameter").find(|p| cx.has_word(p, "="))) {
            cx.error(p.span, "DEFARG-13", "a default argument in a later declaration of a function template");
            return true;
        }
        let earlier = tpl.defaults.clone();
        let Some(merged) = merge_defaults(cx, &earlier, &defaults, span) else { return true };
        cx.u.templates[t].defaults = merged;
        let tpl = &cx.u.templates[t];
        if fd.prod != "function_definition" {
            return true;
        }
        if tpl.node.prod == "function_definition" {
            return false;
        }
        let limit = cx.u.entities.len() + 1;
        let tpl = &mut cx.u.templates[t];
        tpl.params = params.into_iter().map(|p| p.0).collect();
        tpl.node = fd.clone();
        tpl.span = span;
        tpl.limit = limit;
        // The specializations that uses made before the definition get
        // their bodies now.
        let made: Vec<(Vec<TArg>, EntityId)> = tpl.specs.iter().filter(|(_, f)| !tpl.explicit.contains(f)).cloned().collect();
        for (args, f) in made {
            define_later(cx, t, &args, f);
        }
        return true;
    }
    if !found.is_empty() {
        return false;
    }
    let id = cx.u.templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.templates.push(FnTemplate { name: name.clone(), defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), specs: Vec::new(), explicit: Vec::new(), span, limit });
    names::declare_in(cx, saved, &name, EntKind::Template(id), span, Linkage::External);
    true
}

/// TPL-3: a template or a specialization in a C linkage specification.
fn c_linkage(cx: &mut Check, n: &Node) -> bool {
    if cx.lang_c {
        cx.error(n.span, "TPL-3", "a template or a specialization with C language linkage");
    }
    cx.lang_c
}

/// PACK-8: in a function body of a template with no pack, `x...` after a
/// name or a parenthesized expression expands no pack.
fn expands_no_pack(cx: &mut Check, fd: &Node) -> bool {
    let Some(body) = fd.find("compound_stmt") else { return false };
    let mut toks = Vec::new();
    body.tokens(&mut toks);
    // A generic lambda's pack (`auto...`) can be expanded.
    if toks.windows(3).any(|w| cx.toks[w[0]].text == "auto" && (cx.toks[w[1]].text == "..." || cx.toks[w[2]].text == "...")) {
        return false;
    }
    for w in toks.windows(2) {
        let (prev, t) = (cx.toks[w[0]].text.as_str(), cx.toks[w[1]].text.as_str());
        let name = is_word(prev) && !crate::foundations::types::check::is_fundamental(prev) && !matches!(prev, "sizeof" | "auto" | "const" | "volatile");
        if t == "..." && (name || prev == ")") {
            let span = cx.toks[w[1]].span;
            cx.error(span, "PACK-8", "an expansion whose pattern holds no pack");
            return true;
        }
    }
    false
}

/// TPL-18: a declaration in the template of a name that one of its
/// parameters has. A friend function can have the name (TPL-48).
fn shadows_param(cx: &mut Check, n: &Node, params: &[Param]) -> bool {
    fn walk<'n>(n: &'n Node, out: &mut Vec<&'n Node>) {
        for k in &n.kids {
            let Kid::Node(c) = k else { continue };
            match c.prod {
                "declarator_id" => out.push(c),
                "friend_decl" => {}
                _ => walk(c, out),
            }
        }
    }
    let mut ids = Vec::new();
    walk(n, &mut ids);
    let hit = ids.into_iter().find(|d| params.iter().any(|p| p.0.as_ref() == cx.source(d)));
    if let Some(d) = hit {
        cx.error(d.span, "TPL-18", format!("'{}' declared again in the scope of the template parameter of that name", cx.source(d)));
        return true;
    }
    false
}

/// TSP rules: `template <> class C<A...> { ... };` or
/// `template <> T v<A...> = e;`, an explicit specialization of a class
/// template or a variable template of the scope, before any use of those
/// arguments. False for another form, such as a function's.
pub fn explicit_specialization(cx: &mut Check, n: &Node) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() {
        return false;
    }
    if c_linkage(cx, n) {
        return true;
    }
    if let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) {
        return explicit_function(cx, fd);
    }
    let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) else { return false };
    let saved = cx.scope;
    if sd.child("init_declarator").is_none() {
        let Some(ds) = sd.child("decl_specifiers") else { return false };
        let specs: Vec<&Node> = ds.children("decl_specifier").collect();
        let [one] = specs[..] else { return false };
        let Some(cd) = one.child("type_specifier").and_then(|t| t.child("class_def").or_else(|| t.child("union_def"))) else { return false };
        // A `union_def` is its own head (`classes/unions`).
        let head = if cd.prod == "union_def" { cd } else {
            let Some(h) = cd.child("class_head") else { return false };
            h
        };
        if head.child("nested_name").is_some() {
            return false;
        }
        let Some(tid) = head.child("template_id") else { return false };
        let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
        let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
            EntKind::ClassTemplate(v) => Some(v),
            _ => None,
        }) else {
            return false;
        };
        let kinds = cx.u.class_templates[v].kinds.clone();
        let marks = cx.u.diags.len();
        let Some(args) = template_args(cx, tid, &kinds).filter(|_| cx.u.diags.len() == marks) else {
            cx.u.diags.truncate(marks);
            return false;
        };
        // TSP-32: `union` agrees only with `union`.
        let primary = cx.u.class_templates[v].node.clone();
        if (primary.prod == "union_def") != (cd.prod == "union_def") {
            cx.error(head.span, "TSP-32", format!("an explicit specialization of '{}' with a class key that does not agree with the template's", name));
            return true;
        }
        let ct = &cx.u.class_templates[v];
        if ct.specs.iter().any(|(a, c)| *a == args && ct.explicit.contains(c)) {
            return false;
        }
        // TSP-6: a use before the explicit specialization made the class
        // from the primary template.
        if ct.specs.iter().any(|(a, _)| *a == args) {
            cx.error(head.span, "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
            return true;
        }
        let predicted = cx.u.classes.len();
        cx.u.class_templates[v].specs.push((args.clone(), predicted));
        cx.enter(ScopeKind::Template);
        cx.explicit_head = true;
        let t = crate::classes::members::check::class_def(cx, cd);
        cx.explicit_head = false;
        cx.leave(saved);
        match t.class() == Some(predicted) {
            true => cx.u.class_templates[v].explicit.push(predicted),
            false => cx.u.class_templates[v].specs.retain(|(a, _)| *a != args),
        }
        return true;
    }
    let Some((ds, dn, _, Some(tid))) = var_parts(sd) else { return false };
    if dn.find("ptr_operator").is_some() {
        return false;
    }
    // TSP-24: of the storage classes, only `thread_local`.
    if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "static" | "extern" | "register" | "mutable")) {
        cx.error(d.span, "TSP-24", format!("the storage class '{}' on an explicit specialization", cx.source(d)));
        return true;
    }
    let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
    let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::VarTemplate(v) => Some(v),
        _ => None,
    }) else {
        return false;
    };
    let kinds = cx.u.var_templates[v].kinds.clone();
    let marks = cx.u.diags.len();
    let Some(args) = template_args(cx, tid, &kinds).filter(|_| cx.u.diags.len() == marks) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    let vt = &cx.u.var_templates[v];
    if vt.explicit.iter().any(|(a, _)| *a == args) {
        return false;
    }
    if vt.specs.iter().any(|(s, a, _)| s.is_none() && *a == args) {
        cx.error(tid.span, "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
        return true;
    }
    let Some(id) = instantiate_var(cx, v, &[], &[], sd) else { return false };
    cx.u.var_templates[v].explicit.push((args, id));
    true
}

/// TSP-2: `template <> R f<A...>(P...) { ... }`, an explicit
/// specialization of a function template of the scope. The arguments
/// that the template-id leaves out come from the function's type
/// (DED-32). False for another form, such as a declaration that is not
/// a definition.
fn explicit_function(cx: &mut Check, fd: &Node) -> bool {
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    let Some(did) = dn.find("declarator_id") else { return false };
    let tid = did.find("template_id");
    let name = match tid {
        Some(tid) => tid.child("template_name").map(|t| cx.source(t)),
        None => did.find("unqualified_id").map(|u| cx.source(u)),
    };
    let Some(name) = name else { return false };
    let saved = cx.scope;
    let Some(t) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Template(t) => Some(t),
        _ => None,
    }) else {
        return false;
    };
    // TSP-24: of the storage classes, only `thread_local`.
    if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "static" | "extern")) {
        cx.error(d.span, "TSP-24", format!("the storage class '{}' on an explicit specialization", cx.source(d)));
        return true;
    }
    let tpl = cx.u.templates[t].clone();
    let marks = cx.u.diags.len();
    let explicit = match tid {
        Some(tid) => {
            let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
            match template_args(cx, tid, &tpl.kinds[..n.min(tpl.kinds.len())]) {
                Some(a) if n <= tpl.kinds.len() && cx.u.diags.len() == marks => a,
                _ => {
                    cx.u.diags.truncate(marks);
                    return false;
                }
            }
        }
        None => Vec::new(),
    };
    cx.enter(ScopeKind::Template);
    cx.explicit_head = true;
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.explicit_head = false;
    let fty = d.ty.func().cloned();
    let done = |cx: &mut Check| {
        cx.leave(saved);
        cx.u.diags.truncate(marks);
        false
    };
    let Some(fty) = fty.filter(|_| !specs.unsupported && !d.unsupported && d.qual == names::Qual::None && cx.u.diags.len() == marks) else { return done(cx) };
    // DED-32: the function type gives the arguments that the template-id
    // leaves out.
    let mut bound: Vec<Option<QType>> = (0..tpl.params.len()).map(|k| match explicit.get(k) {
        Some(TArg::Type(t)) => Some(t.clone()),
        _ => None,
    }).collect();
    let same_shape = fty.params.len() == tpl.fty.params.len() && fty.variadic == tpl.fty.variadic;
    if !same_shape || !tpl.fty.params.iter().zip(&fty.params).all(|(p, a)| match_exact(&cx.u, p, a, &mut bound)) || !match_exact(&cx.u, &tpl.fty.ret, &fty.ret, &mut bound) {
        return done(cx);
    }
    let args: Option<Vec<TArg>> = (0..tpl.params.len()).map(|k| match &tpl.kinds[k] {
        None => bound[k].clone().map(TArg::Type),
        Some(_) => explicit.get(k).cloned().filter(|a| matches!(a, TArg::Value(..))),
    }).collect();
    let Some(args) = args else { return done(cx) };
    if let Some(&(_, f)) = tpl.specs.iter().find(|(a, _)| *a == args) {
        cx.leave(saved);
        if tpl.explicit.contains(&f) {
            return false;
        }
        cx.error(d.name.as_ref().map(|n| n.1).unwrap_or(dn.span), "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
        return true;
    }
    let body = fd.child("function_body");
    let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, body) else { return done(cx) };
    cx.u.templates[t].specs.push((args, f));
    cx.u.templates[t].explicit.push(f);
    if let Some(cs) = body.and_then(|b| b.child("compound_stmt")) {
        crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
    }
    cx.leave(saved);
    true
}

/// TPL rules: `template <class T> R C<T>::f(P...) { ... }` or
/// `template <class T> T C<T>::s = e;`, a member of a class template
/// defined outside the class. Each specialization of the class, made
/// before or after, gets the member with the parameters bound. None when
/// the declaration has another form.
fn outside_member(cx: &mut Check, n: &Node, params: &[Param]) -> Option<bool> {
    let decl = n.child("declaration")?;
    let (node, dn) = match decl.child("function_definition") {
        Some(fd) => (fd, fd.child("declarator")?),
        None => {
            let bd = decl.child("block_decl")?;
            let sd = bd.child("simple_decl")?;
            let mut decls = sd.children("init_declarator");
            let one = decls.next()?;
            if decls.next().is_some() {
                return None;
            }
            (bd, one.child("declarator")?)
        }
    };
    let nn = dn.find("declarator_id")?.find("qualified_name")?.child("nested_name")?;
    let [Kid::Node(tid), _] = nn.kids.as_slice() else { return None };
    if tid.prod != "template_id" {
        return None;
    }
    let name = cx.source(tid.child("template_name")?);
    let v = names::declared_here(cx, cx.scope, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    })?;
    // The arguments are the parameters in order; another list names a
    // partial specialization's member.
    let args: Vec<String> = tid.child("template_args").map(|ta| ta.children("template_arg").map(|a| cx.source(a)).collect()).unwrap_or_default();
    let kinds: Vec<Option<QType>> = params.iter().map(|p| p.2.clone()).collect();
    if args.len() != params.len() || args.iter().zip(params).any(|(a, p)| a.as_str() != p.0.as_ref()) || kinds != cx.u.class_templates[v].kinds {
        return Some(false);
    }
    if cx.func.is_some() || cx.member_of.is_some() || dependent_forms(cx, node, params) {
        return Some(false);
    }
    // DEFARG-13: no default argument in the definition of a member of a
    // class template outside its class.
    if node.prod == "function_definition" {
        if let Some(p) = dn.find("parameters").and_then(|ps| ps.children("parameter").find(|p| cx.has_word(p, "="))) {
            cx.error(p.span, "DEFARG-13", "a default argument in the definition of a member of a class template outside its class");
            return Some(true);
        }
    }
    let names_: Vec<Rc<str>> = params.iter().map(|p| p.0.clone()).collect();
    let limit = cx.u.entities.len();
    cx.u.class_templates[v].outside.push((names_, node.clone(), limit));
    let k = cx.u.class_templates[v].outside.len() - 1;
    let ct = cx.u.class_templates[v].clone();
    for (args, c) in &ct.specs {
        if !ct.explicit.contains(c) {
            define_outside(cx, v, k, args);
        }
    }
    Some(true)
}

/// The member that outside definition `k` of class template `v` gives
/// the specialization of `args`. An error in it leaves the unit
/// unsupported, as for a function template.
fn define_outside(cx: &mut Check, v: usize, k: usize, args: &[TArg]) {
    let ct = cx.u.class_templates[v].clone();
    let (names_, node, limit) = ct.outside[k].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    let saved_limit = cx.lookup_limit.replace(limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = ct.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &names_, args, ct.span);
    match node.prod {
        "function_definition" => crate::declarations::functions::definitions::check::function_definition(cx, &node),
        _ => {
            crate::declarations::check::block_decl(cx, &node, false);
        }
    }
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        cx.unsupported(node.span, format!("an error in a member of a specialization of the class template '{}' (the first pieces of build step 5 do not tell it from a gap of their model)", ct.name));
    }
}

/// The definition uses a form that needs the dependent names of model
/// sections 1 and 2: `typename`, `template`, `requires`, `decltype`,
/// `concept`, or a name qualified by a template parameter.
fn dependent_forms(cx: &Check, fd: &Node, params: &[Param]) -> bool {
    let mut toks = Vec::new();
    fd.tokens(&mut toks);
    let words: Vec<&str> = toks.iter().map(|&t| cx.toks[t].text.as_str()).collect();
    // DEP-31: a class that is incomplete at the definition and complete
    // at a specialization needs the definition checks.
    let incomplete = words.iter().any(|w| names::lookup(cx, w).iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(c) if !cx.u.classes[c].complete)));
    // `decltype` of an operand that names no parameter is independent.
    let mut dependent_decltype = false;
    for (k, w) in words.iter().enumerate() {
        if *w != "decltype" {
            continue;
        }
        let mut depth = 0;
        for x in &words[k + 1..] {
            match *x {
                "(" => depth += 1,
                ")" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ if params.iter().any(|(p, _, _)| p.as_ref() == *x) => dependent_decltype = true,
                _ => {}
            }
        }
    }
    let _ = dependent_decltype;
    // DEP-12: unqualified lookup skips a dependent base, which the first
    // pieces of build step 5 do not tell from another base.
    let dependent_base = fd.find("base_clause").is_some_and(|bc| {
        let mut ts = Vec::new();
        bc.tokens(&mut ts);
        let words: Vec<&str> = ts.iter().map(|&t| cx.toks[t].text.as_str()).collect();
        // A base `C<..T..>`: a parameter between `<` and `>`.
        let mut depth = 0;
        words.iter().any(|w| {
            match *w {
                "<" => depth += 1,
                ">" => depth -= 1,
                _ => {}
            }
            depth > 0 && params.iter().any(|(p, _, _)| p.as_ref() == *w)
        })
    });
    incomplete
        || dependent_base
        || words.iter().any(|w| matches!(*w, "template" | "requires" | "concept"))
        // A pack expansion or a fold of a parameter (packs wait for a
        // later piece); `catch (...)` and a C ellipsis are not.
        || words.iter().enumerate().any(|(k, w)| *w == "..." && words[k.saturating_sub(3)..k].iter().any(|x| params.iter().any(|(p, _, _)| p.as_ref() == *x)))
        || words.windows(2).any(|w| w[1] == "::" && params.iter().any(|(p, _, _)| p.as_ref() == w[0]))
        // A type named after a template-id qualifier, which can need
        // `typename` (DEP-19).
        || words.windows(4).any(|w| w[0] == ">" && w[1] == "::" && is_word(w[2]) && is_word(w[3]))
}

/// An identifier or a keyword.
fn is_word(w: &str) -> bool {
    w.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_')
}

/// TARG-1, DED-P3: the specialization of the function template `t` for
/// the arguments `args`, made the first time: the definition checked in
/// a scope that binds each parameter to its argument. The function is
/// recorded before its body is checked, so a call of the same
/// specialization in the body finds it. None when the declaration does
/// not check.
pub fn specialize(cx: &mut Check, t: usize, args: &[TArg]) -> Option<EntityId> {
    if let Some(&(_, f)) = cx.u.templates[t].specs.iter().find(|(a, _)| a.as_slice() == args) {
        return Some(f);
    }
    let tpl = cx.u.templates[t].clone();
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_limit = cx.lookup_limit.replace(tpl.limit);
    // The specialization is a function of the template's namespace, made
    // outside the class, lambda, or operand that the call stands in.
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = tpl.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &tpl.params, args, tpl.span);
    let fd = &tpl.node;
    let r = (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, fd.child("decl_specifiers")?, Default::default());
        // A template declared and not defined has a declaration.
        let dn = fd.child("declarator").or_else(|| fd.child("init_declarator").and_then(|i| i.child("declarator")))?;
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        if specs.unsupported || d.unsupported || !d.ty.is_func() {
            return None;
        }
        let body = fd.child("function_body");
        let f = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, body)?;
        cx.u.templates[t].specs.push((args.to_vec(), f));
        let cs = body.and_then(|b| b.child("compound_stmt"));
        if let Some(cs) = cs {
            crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
        }
        Some(f)
    })();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.lookup_limit = saved_limit;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.instantiating -= 1;
    // An error in a specialization can come from a part of the model that
    // the first piece does not have (the points of instantiation, the
    // definition checks), so it leaves the unit unsupported.
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) || cx.depth_exceeded {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        if !depth_failed(cx, tpl.span) {
            cx.unsupported(tpl.span, format!("an error in a specialization of the function template '{}' (the first piece of build step 5 does not tell it from a gap of its model)", tpl.name));
        }
        return None;
    }
    r
}

/// TR-13: a chain of instantiations went past the limit. Inside the
/// chain the failure is quiet; the use that began it reports the error.
fn depth_failed(cx: &mut Check, span: Span) -> bool {
    if !cx.depth_exceeded {
        return false;
    }
    if cx.instantiating == 0 {
        cx.depth_exceeded = false;
        let limit = cx.depth_limit;
        cx.error(span, "TR-13", format!("the instantiations nest deeper than the limit of {}", limit));
    }
    true
}

/// The body of the specialization `f` of the function template `t`,
/// which a use declared before the template's definition.
fn define_later(cx: &mut Check, t: usize, args: &[TArg], f: EntityId) {
    let tpl = cx.u.templates[t].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_limit = cx.lookup_limit.replace(tpl.limit);
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = tpl.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &tpl.params, args, tpl.span);
    let fd = &tpl.node;
    (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, fd.child("decl_specifiers")?, Default::default());
        let d = crate::declarations::declarators::check::declarator(cx, &specs, fd.child("declarator")?);
        let cs = fd.child("function_body").and_then(|b| b.child("compound_stmt"))?;
        crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
        Some(())
    })();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.lookup_limit = saved_limit;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.instantiating -= 1;
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        cx.unsupported(tpl.span, format!("an error in a specialization of the function template '{}' (the first piece of build step 5 does not tell it from a gap of its model)", tpl.name));
    }
}

/// The function templates that a call by name finds, after their
/// specializations for the call's arguments are made: each template
/// whose arguments deduce (DED-P3), with the explicit arguments first.
pub fn call_candidates(cx: &mut Check, found: &[EntityId], tid: Option<&Node>, args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    let mut out = Vec::new();
    for &e in found {
        let EntKind::Template(t) = cx.u.entities[e].kind else { continue };
        // The explicit arguments, which fill the first parameters.
        let explicit = match tid {
            Some(tid) => {
                let kinds = cx.u.templates[t].kinds.clone();
                let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
                match template_args(cx, tid, &kinds[..n.min(kinds.len())]) {
                    Some(a) if n <= kinds.len() => a,
                    _ => continue,
                }
            }
            None => Vec::new(),
        };
        let Some(partial) = crate::templates::deduction::check::deduce_call(cx, t, &explicit, args) else { continue };
        // TPL-13: a parameter that deduction leaves takes its default.
        let tpl = cx.u.templates[t].clone();
        let mut bound: Vec<TArg> = Vec::new();
        for (k, a) in partial.iter().enumerate() {
            match a {
                Some(a) => bound.push(a.clone()),
                None if fill_defaults(cx, &tpl.kinds[..=k], &tpl.defaults, &tpl.params, tpl.scope, tpl.limit, &mut bound, tpl.span) => {}
                None => break,
            }
        }
        if bound.len() < partial.len() {
            continue;
        }
        if let Some(f) = specialize(cx, t, &bound) {
            out.push(f);
        }
    }
    out
}

/// The function and class templates of the unit that nothing
/// specialized: their definitions are not checked yet, so the unit is
/// unsupported.
pub fn end_of_unit(cx: &mut Check) {
    // The templates that a chain past the limit used are specialized.
    if cx.u.diags.iter().any(|d| d.rule == Some("TR-13")) {
        return;
    }
    let mut unused: Vec<(Rc<str>, Span)> = Vec::new();
    for t in 0..cx.u.templates.len() {
        let tpl = &cx.u.templates[t];
        if tpl.specs.iter().all(|(_, f)| tpl.explicit.contains(f)) && !trial(cx, Some(t), None) {
            unused.push((cx.u.templates[t].name.clone(), cx.u.templates[t].span));
        }
    }
    // An explicit specialization does not check the primary's definition.
    for v in 0..cx.u.class_templates.len() {
        let ct = &cx.u.class_templates[v];
        if ct.defined && ct.specs.iter().all(|(_, c)| ct.explicit.contains(c)) && !trial(cx, None, Some(v)) {
            unused.push((cx.u.class_templates[v].name.clone(), cx.u.class_templates[v].span));
        }
    }
    for (name, span) in unused {
        cx.unsupported(span, format!("the template '{}', which nothing specializes (its definition is not checked yet)", name));
    }
}

/// A trial specialization of an unused function template `t` or class
/// template `v` with `int` for each type parameter. A definition that it
/// checks with no diagnostic has no error that is independent of the
/// arguments. The trial leaves no diagnostic and no static object.
fn trial(cx: &mut Check, t: Option<usize>, v: Option<usize>) -> bool {
    let kinds = match (t, v) {
        (Some(t), _) => cx.u.templates[t].kinds.clone(),
        (_, Some(v)) => cx.u.class_templates[v].kinds.clone(),
        _ => return false,
    };
    if kinds.iter().any(|k| k.is_some()) {
        return false;
    }
    let args: Vec<TArg> = kinds.iter().map(|_| TArg::Type(QType::new(Type::Int(crate::sem::types::IntK::Int)))).collect();
    let marks = cx.u.diags.len();
    let statics = cx.u.statics.len();
    let ok = match (t, v) {
        (Some(t), _) => !cx.u.templates[t].specs.iter().any(|(a, _)| *a == args) && specialize(cx, t, &args).is_some(),
        (_, Some(v)) => !cx.u.class_templates[v].specs.iter().any(|(a, _)| *a == args) && instantiate_class(cx, v, &args, None).is_some(),
        _ => false,
    };
    let clean = ok && cx.u.diags.len() == marks;
    cx.u.diags.truncate(marks);
    cx.u.statics.truncate(statics);
    cx.depth_exceeded = false;
    clean
}

/// The declarator, the initializer, and the template-id of the name of a
/// variable template's declaration with one declarator, if it has that
/// form.
fn var_parts(sd: &Node) -> Option<(&Node, &Node, &Node, Option<&Node>)> {
    let ds = sd.child("decl_specifiers")?;
    let mut decls = sd.children("init_declarator");
    let idecl = decls.next()?;
    if decls.next().is_some() {
        return None;
    }
    let dn = idecl.child("declarator")?;
    let init = idecl.child("init")?;
    let tid = dn.find("declarator_id").and_then(|d| d.find("template_id"));
    Some((ds, dn, init, tid))
}

/// TPL rules: a variable template at namespace scope whose parameters are
/// types, with one declarator and an initializer, or a partial
/// specialization of one (TSP-9). False for another form.
fn var_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>) -> bool {
    let Some((_, dn, init, tid)) = var_parts(sd) else { return false };
    if dn.find("ptr_operator").is_some() || dependent_forms(cx, init, &params) {
        return false;
    }
    if tid.is_some() && defaults.iter().any(|d| d.is_some()) {
        cx.error(dn.span, "TSP-11", "a default template argument on a partial specialization");
        return true;
    }
    if !defaults_last(cx, &defaults, dn.span) {
        return true;
    }
    let saved = cx.scope;
    match tid {
        None => {
            let Some(name) = dn.find("declarator_id").and_then(|d| d.find("unqualified_id")).map(|u| cx.source(u)) else { return false };
            if !names::declared_here(cx, saved, &name).is_empty() {
                return false;
            }
            let id = cx.u.var_templates.len();
            let limit = cx.u.entities.len() + 1;
            cx.u.var_templates.push(crate::sem::VarTemplate { name: name.as_str().into(), defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: sd.clone(), scope: saved, partials: Vec::new(), specs: Vec::new(), explicit: Vec::new(), span: dn.span, limit });
            names::declare_in(cx, saved, &name, EntKind::VarTemplate(id), dn.span, Linkage::External);
            true
        }
        Some(tid) => {
            // TSP-9: a partial specialization of a primary of the scope.
            let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
            let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
                EntKind::VarTemplate(v) => Some(v),
                _ => None,
            }) else {
                return false;
            };
            // A partial specialization with a constant parameter waits for
            // a later piece.
            if params.iter().any(|p| p.2.is_some()) || cx.u.var_templates[v].kinds.iter().any(|k| k.is_some()) {
                return false;
            }
            let ts = cx.enter(ScopeKind::Template);
            declare_params(cx, ts, &params);
            let marks = cx.u.diags.len();
            let pattern = template_type_args(cx, tid);
            cx.leave(saved);
            let Some(pattern) = pattern.filter(|_| cx.u.diags.len() == marks) else {
                cx.u.diags.truncate(marks);
                return false;
            };
            if pattern.len() != cx.u.var_templates[v].params.len() {
                return false;
            }
            cx.u.var_templates[v].partials.push((params.into_iter().map(|p| p.0).collect(), pattern, sd.clone()));
            true
        }
    }
}

/// The type arguments of a template-id, or None when one is not a type.
fn template_type_args(cx: &mut Check, tid: &Node) -> Option<Vec<QType>> {
    let mut out = Vec::new();
    if let Some(ta) = tid.child("template_args") {
        for a in ta.children("template_arg") {
            let t = a.child("type_id")?;
            out.push(crate::declarations::declarators::check::type_id(cx, t));
        }
    }
    Some(out)
}

fn match_exact(u: &crate::sem::Unit, p: &QType, a: &QType, bound: &mut [Option<QType>]) -> bool {
    crate::templates::deduction::check::match_exact(u, p, a, bound)
}

/// TARG-1, TSP-13 to TSP-15: `v<A...>`, a specialization of a variable
/// template: the one partial specialization whose pattern the arguments
/// match, or the primary. None when the name is no variable template.
pub fn var_template_use(cx: &mut Check, tid: &Node) -> Option<crate::sem::tree::Expr> {
    use crate::sem::tree::{Cat, EK, Expr};
    let span = tid.span;
    let name = cx.source(tid.child("template_name")?);
    let v = names::lookup(cx, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::VarTemplate(v) => Some(v),
        _ => None,
    })?;
    let unsupported = |cx: &mut Check, why: &str| {
        let t = cx.unsupported(span, why.to_string());
        Some(Expr::new(EK::Int(0), t, Cat::PR, span))
    };
    let vt = cx.u.var_templates[v].clone();
    let marks = cx.u.diags.len();
    let Some(args) = args_with_defaults(cx, tid, &vt.kinds, &vt.defaults, &vt.params, vt.scope, vt.limit) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return Some(crate::expressions::check::bad(span));
        }
        return unsupported(cx, "a variable template with arguments of another number or form");
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(crate::expressions::check::bad(span));
    }
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        return unsupported(cx, "a variable template with a dependent argument");
    }
    // TSP rules: an explicit specialization is the variable of its
    // arguments.
    if let Some(&(_, var)) = vt.explicit.iter().find(|(a, _)| *a == args) {
        crate::declarations::initialization::forms::check::note_use(cx, var, span);
        let ty = cx.u.var(var)?.ty.clone();
        return Some(Expr::new(EK::Var(var), ty, Cat::L, span));
    }
    let types: Vec<QType> = args.iter().filter_map(|a| match a {
        TArg::Type(t) => Some(t.clone()),
        TArg::Value(..) => None,
    }).collect();
    let mut matches = Vec::new();
    for (k, (ps, pattern, _)) in vt.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        if types.len() == args.len() && pattern.iter().zip(&types).all(|(p, a)| match_exact(&cx.u, p, a, &mut bound)) {
            if let Some(b) = bound.into_iter().map(|b| b.map(TArg::Type)).collect::<Option<Vec<TArg>>>() {
                matches.push((k, b));
            }
        }
    }
    if matches.len() > 1 {
        return unsupported(cx, "a variable template whose arguments match several partial specializations (partial ordering)");
    }
    let (source, bound) = match matches.pop() {
        Some((k, b)) => (Some(k), b),
        None => (None, args.clone()),
    };
    let var = match vt.specs.iter().find(|(s, a, _)| *s == source && *a == bound) {
        Some(&(_, _, id)) => id,
        None => {
            let (params, node) = match source {
                Some(k) => (vt.partials[k].0.clone(), vt.partials[k].2.clone()),
                None => (vt.params.clone(), vt.node.clone()),
            };
            match instantiate_var(cx, v, &params, &bound, &node) {
                Some(id) => {
                    cx.u.var_templates[v].specs.push((source, bound, id));
                    id
                }
                None if depth_failed(cx, span) => return Some(crate::expressions::check::bad(span)),
                None => return unsupported(cx, "a specialization of a variable template that the first pieces of build step 5 do not make"),
            }
        }
    };
    crate::declarations::initialization::forms::check::note_use(cx, var, span);
    let ty = cx.u.var(var)?.ty.clone();
    Some(Expr::new(EK::Var(var), ty, Cat::L, span))
}

/// The variable of a specialization: the declaration `node` checked with
/// each of `params` bound to its argument in `args`. An error in it
/// leaves the unit unsupported, as for a function template.
fn instantiate_var(cx: &mut Check, v: usize, params: &[Rc<str>], args: &[TArg], node: &Node) -> Option<EntityId> {
    let vt = cx.u.var_templates[v].clone();
    let (ds, _, init, _) = var_parts(node)?;
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_limit = cx.lookup_limit.replace(vt.limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = vt.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, params, args, vt.span);
    let r = (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
        if specs.unsupported || specs.placeholder.is_some() {
            return None;
        }
        let ty = specs.ty.clone()?;
        let ty = if specs.constexpr { ty.with_cv(true, ty.v) } else { ty };
        let i = crate::declarations::initialization::forms::check::initializer(cx, &ty.unqual(), init, &vt.name)?;
        let var = crate::sem::Var { ty, storage: crate::sem::Storage::Static, constexpr: specs.constexpr, constinit: false, inline: true, is_extern: false, is_static: false, param: false, defined: true, init: Some(i), const_init: specs.constexpr, used: false, maybe_unused: false, align: None };
        let id = cx.u.add(crate::sem::Entity { name: vt.name.clone(), kind: EntKind::Var(var), span: vt.span, scope: ts, linkage: Linkage::External, deprecated: None });
        cx.u.statics.push(id);
        Some(id)
    })();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error || d.sev == crate::diag::Severity::Unsupported) || cx.depth_exceeded {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    r
}

/// TPL rules: a class template definition at namespace scope whose
/// parameters are types. False for another form, such as a declaration
/// that is not a definition.
fn class_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>) -> bool {
    let Some(ds) = sd.child("decl_specifiers") else { return false };
    let specs: Vec<&Node> = ds.children("decl_specifier").collect();
    let [one] = specs[..] else { return false };
    let saved = cx.scope;
    // `template <class T> struct C;` declares the template.
    if let Some(et) = one.child("type_specifier").and_then(|t| t.child("elaborated_type")) {
        if et.child("nested_name").is_some() || et.find("template_id").is_some() || cx.first_word(et.child("class_key").unwrap_or(et)) == "enum" {
            return false;
        }
        let Some(name) = cx.words(et).last().map(|w| w.to_string()) else { return false };
        let found = names::declared_here(cx, saved, &name);
        if let Some(v) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
            EntKind::ClassTemplate(v) => Some(v),
            _ => None,
        }) {
            if cx.u.class_templates[v].kinds != params.iter().map(|p| p.2.clone()).collect::<Vec<_>>() {
                return false;
            }
            let earlier = cx.u.class_templates[v].defaults.clone();
            if let Some(m) = merge_defaults(cx, &earlier, &defaults, et.span) {
                if defaults_last(cx, &m, et.span) {
                    cx.u.class_templates[v].defaults = m;
                }
            }
            return true;
        }
        if !defaults_last(cx, &defaults, et.span) {
            return true;
        }
        if !found.is_empty() {
            return false;
        }
        let id = cx.u.class_templates.len();
        let limit = cx.u.entities.len() + 1;
        cx.u.class_templates.push(crate::sem::ClassTemplate { name: name.as_str().into(), defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: et.clone(), scope: saved, specs: Vec::new(), explicit: Vec::new(), outside: Vec::new(), partials: Vec::new(), defined: false, span: et.span, limit });
        names::declare_in(cx, saved, &name, EntKind::ClassTemplate(id), et.span, Linkage::External);
        return true;
    }
    let Some(cd) = one.child("type_specifier").and_then(|t| t.child("class_def").or_else(|| t.child("union_def"))) else { return false };
    // A `union_def` is its own head (`classes/unions`).
    let head = if cd.prod == "union_def" { cd } else {
        let Some(h) = cd.child("class_head") else { return false };
        h
    };
    if head.child("nested_name").is_some() {
        return false;
    }
    if let Some(tid) = head.child("template_id") {
        // TSP-11: a partial specialization's parameters have no default.
        if defaults.iter().any(|d| d.is_some()) {
            cx.error(head.span, "TSP-11", "a default template argument on a partial specialization");
            return true;
        }
        return class_partial(cx, cd, tid, params);
    }
    let Some(name) = head.child("class_name").map(|c| cx.source(c)) else { return false };
    if shadows_param(cx, cd, &params) {
        return true;
    }
    if dependent_forms(cx, cd, &params) {
        return false;
    }
    // The definition of a template declared before.
    let found = names::declared_here(cx, saved, &name);
    if let Some(v) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    }) {
        let ct = &cx.u.class_templates[v];
        if ct.defined || ct.kinds != params.iter().map(|p| p.2.clone()).collect::<Vec<_>>() || !ct.specs.is_empty() {
            return false;
        }
        let earlier = ct.defaults.clone();
        let Some(merged) = merge_defaults(cx, &earlier, &defaults, head.span) else { return true };
        if !defaults_last(cx, &merged, head.span) {
            return true;
        }
        let limit = cx.u.entities.len() + 1;
        let ct = &mut cx.u.class_templates[v];
        ct.defaults = merged;
        ct.params = params.into_iter().map(|p| p.0).collect();
        ct.node = cd.clone();
        ct.span = head.span;
        ct.limit = limit;
        ct.defined = true;
        return true;
    }
    if !found.is_empty() {
        return false;
    }
    // LSUP rules: `std::initializer_list`, which the language builds
    // braced lists into (ILC rules), and `std::allocator`, whose
    // allocation constant evaluation allows (LSUP-11), are classes that
    // the first pieces of build step 5 do not model.
    if matches!(&*name, "initializer_list" | "allocator") && cx.u.scopes[saved].name.as_ref() == "std" {
        return false;
    }
    if !defaults_last(cx, &defaults, head.span) {
        return true;
    }
    let id = cx.u.class_templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.class_templates.push(crate::sem::ClassTemplate { name: name.as_str().into(), defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: cd.clone(), scope: saved, specs: Vec::new(), explicit: Vec::new(), outside: Vec::new(), partials: Vec::new(), defined: true, span: head.span, limit });
    names::declare_in(cx, saved, &name, EntKind::ClassTemplate(id), head.span, Linkage::External);
    true
}

/// TSP-9: `template <class T> struct C<P...> { ... };`, a partial
/// specialization of a class template of the scope whose parameters are
/// types.
fn class_partial(cx: &mut Check, cd: &Node, tid: &Node, params: Vec<Param>) -> bool {
    let saved = cx.scope;
    let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
    let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    }) else {
        return false;
    };
    if shadows_param(cx, cd, &params) {
        return true;
    }
    if params.iter().any(|p| p.2.is_some()) || cx.u.class_templates[v].kinds.iter().any(|k| k.is_some()) || dependent_forms(cx, cd, &params) {
        return false;
    }
    // TSP-32: `union` agrees only with `union`.
    let ct = &cx.u.class_templates[v];
    let primary_union = ct.node.prod == "union_def" || (ct.node.prod == "elaborated_type" && cx.first_word(ct.node.child("class_key").unwrap_or(&ct.node)) == "union");
    if primary_union != (cd.prod == "union_def") {
        return false;
    }
    let ts = cx.enter(ScopeKind::Template);
    declare_params(cx, ts, &params);
    let marks = cx.u.diags.len();
    let pattern = template_type_args(cx, tid);
    cx.leave(saved);
    let Some(pattern) = pattern.filter(|_| cx.u.diags.len() == marks) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    if pattern.len() != cx.u.class_templates[v].params.len() || !cx.u.class_templates[v].specs.is_empty() {
        return false;
    }
    // TSP-11: each parameter is deducible from the pattern, and a pattern
    // of the parameters alone, in any order, matches every list.
    let index = |t: &QType| match &t.t {
        Type::Dep(d) if !t.c && !t.v => match &**d {
            DepType::Param { index, .. } => Some(*index),
            _ => None,
        },
        _ => None,
    };
    let bare: Vec<Option<u32>> = pattern.iter().map(index).collect();
    let mut seen: Vec<u32> = bare.iter().flatten().copied().collect();
    seen.sort();
    seen.dedup();
    if bare.iter().all(|b| b.is_some()) && seen.len() == bare.len() {
        cx.error(tid.span, "TSP-11", format!("a partial specialization of '{}' that matches every argument list of the template", name));
        return true;
    }
    if let Some(k) = (0..params.len()).find(|&k| !pattern.iter().any(|p| mentions(p, k as u32))) {
        cx.error(tid.span, "TSP-11", format!("the parameter '{}' of a partial specialization of '{}' does not appear in its arguments", params[k].0, name));
        return true;
    }
    cx.u.class_templates[v].partials.push((params.into_iter().map(|p| p.0).collect(), pattern, cd.clone()));
    true
}

/// The type `t` names the parameter `index`.
fn mentions(t: &QType, index: u32) -> bool {
    match &t.t {
        Type::Dep(d) => match &**d {
            DepType::Param { index: i, .. } => *i == index,
            DepType::Spec { args, .. } => args.iter().any(|a| matches!(a, TArg::Type(t) if mentions(t, index))),
            DepType::Opaque { .. } => false,
        },
        Type::Ptr(x) | Type::Ref(x, _) | Type::Array(x, _) | Type::MemPtr(_, x) => mentions(x, index),
        Type::Func(f) => mentions(&f.ret, index) || f.params.iter().any(|p| mentions(p, index)),
        _ => false,
    }
}

/// TARG-1: `C<A...>`, the class of a specialization of a class template
/// with type arguments, made the first time. None when the name is no
/// class template.
pub fn class_template_use(cx: &mut Check, tid: &Node) -> Option<QType> {
    class_template_use_in(cx, tid, None)
}

/// `class_template_use` for a template-id after the qualifier that names
/// the scope `scope`.
pub fn class_template_use_in(cx: &mut Check, tid: &Node, scope: Option<crate::sem::ScopeId>) -> Option<QType> {
    let span = tid.span;
    let name = cx.source(tid.child("template_name")?);
    let found = match scope {
        Some(s) => names::lookup_in(cx, s, &name),
        None => names::lookup(cx, &name),
    };
    // Inside a specialization, the class's own name finds the class,
    // which leads to its template (DEP-15).
    let v = found.into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        EntKind::Class(c) => cx.u.class_templates.iter().position(|t| t.specs.iter().any(|&(_, k)| k == c)),
        _ => None,
    })?;
    let ct = cx.u.class_templates[v].clone();
    let marks = cx.u.diags.len();
    let Some(args) = args_with_defaults(cx, tid, &ct.kinds, &ct.defaults, &ct.params, ct.scope, ct.limit) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return Some(QType::error());
        }
        return Some(cx.unsupported(span, "a class template with arguments of another number or form"));
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(QType::error());
    }
    // In a template's definition, a specialization with a parameter in
    // its arguments is a dependent type.
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        let name: Rc<str> = cx.source(tid).into();
        return Some(QType::new(Type::Dep(Rc::new(DepType::Spec { template: v, args, name }))));
    }
    if let Some(&(_, c)) = ct.specs.iter().find(|(a, _)| *a == args) {
        return Some(QType::new(Type::Class(c)));
    }
    // TSP-13 to TSP-15: the one partial specialization whose pattern the
    // arguments match, or the primary.
    let mut matches = Vec::new();
    for (k, (ps, pattern, _)) in ct.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        if pattern.len() == args.len() && pattern.iter().zip(&args).all(|(p, a)| matches!(a, TArg::Type(a) if match_exact(&cx.u, p, a, &mut bound))) {
            if let Some(b) = bound.into_iter().map(|b| b.map(TArg::Type)).collect::<Option<Vec<TArg>>>() {
                matches.push((k, b));
            }
        }
    }
    if matches.len() > 1 {
        return Some(cx.unsupported(span, "a class template whose arguments match several partial specializations (partial ordering)"));
    }
    let source = matches.pop();
    if source.is_none() && !ct.defined {
        return Some(cx.unsupported(span, format!("a specialization of the class template '{}', which is declared and not defined", name)));
    }
    // INST rules: a member declared with a parameter that a function type
    // binds is a data member of function type, which the first pieces of
    // build step 5 do not tell from a member function.
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_func())) {
        return Some(cx.unsupported(span, "a class template with a function type as an argument"));
    }
    match instantiate_class(cx, v, &args, source) {
        Some(c) => Some(QType::new(Type::Class(c))),
        None if depth_failed(cx, span) => Some(QType::error()),
        None => Some(cx.unsupported(span, format!("a specialization of the class template '{}' that the first pieces of build step 5 do not make", name))),
    }
}

/// The class of a specialization: the definition checked with each
/// parameter bound to its argument, as a class of the template's
/// namespace. The class is recorded before its body, so the body finds
/// it by the template-id. An error in it leaves the unit unsupported.
fn instantiate_class(cx: &mut Check, v: usize, args: &[TArg], source: Option<(usize, Vec<TArg>)>) -> Option<crate::sem::types::ClassId> {
    let ct = cx.u.class_templates[v].clone();
    // A partial specialization binds its own parameters.
    let (params, bound, node) = match &source {
        Some((k, b)) => (ct.partials[*k].0.clone(), b.clone(), ct.partials[*k].2.clone()),
        None => (ct.params.clone(), args.to_vec(), ct.node.clone()),
    };
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    let saved_limit = cx.lookup_limit.replace(ct.limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = ct.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &params, &bound, ct.span);
    // The class that `class_def` makes first is the specialization.
    let predicted = cx.u.classes.len();
    cx.u.class_templates[v].specs.push((args.to_vec(), predicted));
    cx.explicit_head = source.is_some();
    let t = crate::classes::members::check::class_def(cx, &node);
    cx.explicit_head = false;
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    let bad = cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error || d.sev == crate::diag::Severity::Unsupported);
    if t.class() != Some(predicted) || bad || cx.depth_exceeded {
        cx.u.class_templates[v].specs.retain(|(a, _)| a.as_slice() != args);
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    if source.is_some() {
        cx.u.class_templates[v].explicit.push(predicted);
        return Some(predicted);
    }
    for k in 0..cx.u.class_templates[v].outside.len() {
        define_outside(cx, v, k, args);
    }
    Some(predicted)
}
