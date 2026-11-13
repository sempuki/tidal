//! The check part of `expressions/lambdas`. A lambda expression makes a
//! closure type: a class with a data member for each capture and a call
//! operator whose body is the lambda's body (LAM-1, LAM-2). The value of
//! the expression is a closure object that the captures initialize in
//! the order of their members (LAM-4, LAM-28).
//!
//! A capture by copy is a data member of the captured type. A capture by
//! reference is a data member that points to the variable, which a use
//! names through the pointer (LAM-20 leaves the form of such a member to
//! the implementation). Generic lambdas, captures of `this`, and pack
//! captures are unsupported.

use crate::expressions::check::{bad, expr, first_node};
use crate::foundations::names::check as names;
use crate::foundations::values::check::rvalue;
use crate::sem::check::{Check, LambdaCtx};
use crate::sem::tree::{Cat, Expr, Init, EK};
use crate::sem::types::{ClassId, FnType, QType, Type};
use crate::sem::{ClassInfo, ClassKey, Closure, EntKind, EntityId, Field, Func, Linkage, ScopeKind, Special, Storage, Var};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The name of a closure type's call operator, which no identifier
/// spells.
pub const CALL: &str = "operator()";

fn unsupported(cx: &mut Check, span: Span, why: &str) -> Expr {
    let t = cx.unsupported(span, why.to_string());
    Expr::new(EK::Int(0), t, Cat::PR, span)
}

fn local_var(ty: QType) -> Var {
    Var {
        ty,
        storage: Storage::Automatic,
        constexpr: false,
        constinit: false,
        inline: false,
        is_extern: false,
        is_static: false,
        param: false,
        defined: true,
        init: None,
        const_init: false,
        used: true,
        maybe_unused: false,
        align: None,
    }
}

/// The function being checked is the call operator of a closure type.
pub fn in_lambda_body(cx: &Check) -> bool {
    cx.func.as_ref().and_then(|f| f.id).and_then(|id| cx.u.func(id)).and_then(|f| f.class).is_some_and(|c| cx.u.closures.contains_key(&c))
}

/// One capture of the capture list, before the body.
struct Written {
    name: Rc<str>,
    span: Span,
    by_ref: bool,
    init: Option<Node>,
}

/// `lambda_expr`.
pub fn lambda_expr(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    if n.child("lambda_template").is_some() {
        return unsupported(cx, span, "a lambda with a template head (a generic lambda, build step 5)");
    }
    if n.child("requires_clause").is_some() {
        return unsupported(cx, span, "a lambda with a requires clause (build step 5)");
    }
    let Some(body) = n.child("compound_stmt") else { return bad(span) };
    let specs: Vec<&str> = n.children("lambda_specifier").map(|s| cx.first_word(s)).collect();
    let mutable = specs.contains(&"mutable");
    let is_static = specs.contains(&"static");
    let consteval = specs.contains(&"consteval");
    // The capture list.
    let mut default = None;
    let mut written: Vec<Written> = Vec::new();
    if let Some(cs) = n.child("captures") {
        if let Some(d) = cs.child("capture_default") {
            default = Some(cx.first_word(d) == "&");
        }
        for c in cs.children("capture") {
            let words = cx.words(c);
            if words.contains(&"this") {
                return unsupported(cx, c.span, "a capture of this (build step 4)");
            }
            if words.contains(&"...") {
                return unsupported(cx, c.span, "a capture of a pack (build step 5)");
            }
            let Some(id) = c.kids.iter().find(|k| matches!(k, Kid::Tok(_)) && !matches!(cx.text(k), "&" | "...")) else {
                return unsupported(cx, c.span, "a capture of this form");
            };
            let id_span = cx.token(id).map(|t| t.span).unwrap_or(c.span);
            written.push(Written { name: cx.text(id).into(), span: id_span, by_ref: words.contains(&"&"), init: c.child("init").cloned() });
        }
    }
    // LAM-6: a capture list names each entity once.
    for (k, w) in written.iter().enumerate() {
        if written[..k].iter().any(|x| x.name == w.name) {
            cx.error(w.span, "LAM-6", format!("'{}' is captured twice", w.name));
            return bad(span);
        }
    }
    // LAM-6, LAM-17: a lambda outside a block has no capture default and
    // no simple capture; one in a default argument captures nothing.
    let in_block = !cx.default_arg && {
        let mut s = Some(cx.scope);
        let mut found = false;
        while let Some(x) = s {
            match cx.u.scopes[x].kind {
                ScopeKind::Block | ScopeKind::Function => {
                    found = true;
                    break;
                }
                // A default member initializer (LAM-6, LAM-8).
                ScopeKind::Class => {
                    found = cx.func.as_ref().is_some_and(|f| f.id.is_none() && f.this.is_some());
                    break;
                }
                ScopeKind::Namespace => break,
                _ => s = cx.u.scopes[x].parent,
            }
        }
        found
    };
    if !in_block {
        let simple = written.iter().find(|w| w.init.is_none()).map(|w| w.span);
        if cx.default_arg && (default.is_some() || !written.is_empty()) {
            cx.error_cause(span, "LAM-6", Some("LAM-17"), "a lambda in a default argument has a capture");
            return bad(span);
        }
        if default.is_some() {
            cx.error(span, "LAM-6", "a capture default on a lambda that is not in a block");
            return bad(span);
        }
        if let Some(s) = simple {
            cx.error(s, "LAM-6", "a simple capture on a lambda that is not in a block");
            return bad(span);
        }
    }
    // LAM-27: a static lambda has no captures.
    if is_static && (default.is_some() || !written.is_empty()) {
        cx.error(span, "LAM-27", "a static lambda has a capture");
        return bad(span);
    }
    if is_static && mutable {
        cx.error(span, "LAM-27", "a static lambda is mutable");
        return bad(span);
    }
    // The closure type, with its member scope inside the current scope.
    let scope = cx.u.new_scope(ScopeKind::Class, Some(cx.scope), "");
    let class = cx.u.classes.len();
    cx.u.classes.push(ClassInfo { name: format!("(lambda at line {})", span.line).into(), key: ClassKey::Class, scope, fields: Vec::new(), complete: false });
    let mut ctx = LambdaCtx { class, default, mutable, caps: Vec::new(), init_vars: Vec::new(), inits: Vec::new(), constants: Vec::new() };
    // The explicit captures, initialized in the enclosing context.
    for w in &written {
        if let Some(init) = &w.init {
            if !init_capture(cx, &mut ctx, w, init) {
                return bad(span);
            }
            continue;
        }
        // LAM-6: a simple capture uses the other mode than the default.
        if default == Some(w.by_ref) {
            cx.error(w.span, "LAM-6", format!("the capture of '{}' has the mode of the capture default", w.name));
            return bad(span);
        }
        let found = names::lookup(cx, &w.name);
        let Some(&v) = found.first() else {
            cx.not_found(w.span, "NAM-8", &w.name);
            return bad(span);
        };
        if matches!(cx.u.entities[v].kind, EntKind::Field(_)) {
            cx.error(w.span, "LAM-17", format!("the data member '{}' is captured by its name", w.name));
            return bad(span);
        }
        let Some(var) = cx.u.var(v).cloned() else {
            cx.error(w.span, "LAM-6", format!("'{}' is not a variable", w.name));
            return bad(span);
        };
        if var.storage != Storage::Automatic || !cx.u.is_local(cx.u.entities[v].scope) {
            cx.error(w.span, "LAM-6", format!("'{}' does not have automatic storage", w.name));
            return bad(span);
        }
        if cx.bindings.contains(&v) {
            return unsupported(cx, w.span, "a capture of a structured binding");
        }
        let Some(e) = enclosing_use(cx, v, w.span) else { return bad(span) };
        add_capture(cx, &mut ctx, v, w.by_ref, e);
    }
    // The parameters, declared as those of any function (LAM-18).
    let (params, variadic) = crate::declarations::declarators::check::parameters(cx, n.child("parameters"));
    // LAM-17: a parameter does not have the name of an explicit capture.
    for p in &params {
        if let Some((pn, ps)) = &p.name {
            if written.iter().any(|w| &w.name == pn) {
                cx.error(*ps, "LAM-17", format!("the parameter '{}' has the name of a capture", pn));
                return bad(span);
            }
        }
    }
    let defaults = crate::declarations::functions::definitions::check::default_arguments(cx, &params);
    let noexcept = n.child("noexcept_specifier").is_some() && crate::declarations::exception_specifications::check::exception_spec(cx, n, &params);
    let trailing = n.child("type_id").map(|t| crate::declarations::declarators::check::type_id(cx, t));
    if params.iter().any(|p| p.ty.is_error()) || trailing.as_ref().is_some_and(|t| t.is_error()) {
        return bad(span);
    }
    let deduce = trailing.is_none();
    let fty = FnType { ret: trailing.clone().unwrap_or_else(QType::error), params: params.iter().map(|p| p.ty.clone()).collect(), variadic, noexcept, qualified: false };
    // LAM-3, LAM-12: the call operator is const unless the lambda is
    // mutable, and constexpr when its body permits; constant evaluation
    // stops at what the body does not permit.
    let op = Func {
        ty: Rc::new(fty.clone()),
        params: Vec::new(),
        defaults,
        body: None,
        constexpr: !consteval,
        consteval,
        inline: true,
        is_static: false,
        deleted: false,
        noreturn: false,
        nodiscard: false,
        c_linkage: false,
        pending_return: deduce,
        placeholder: deduce,
        used: true,
        defined: true,
        unsupported: None,
        class: Some(class),
        special: Special::None,
        explicit: false,
        member_static: is_static,
        obj_cv: (!mutable && !is_static, false),
    };
    let op = names::declare_in(cx, scope, CALL, EntKind::Func(op), span, Linkage::None);
    cx.u.access.insert(op, crate::sem::Access::Public);
    cx.u.closures.insert(class, Closure { op, captures: 0 });
    let d = crate::declarations::declarators::check::Declared {
        name: Some((CALL.into(), span)),
        qual: names::Qual::None,
        ty: QType::new(Type::Func(Rc::new(fty))),
        params,
        span,
        unsupported: false,
        placeholder: deduce.then(|| crate::declarations::auto::check::Placeholder { decltype: false, c: false, v: false, span }),
        trailing: !deduce,
        attrs: Default::default(),
        node: n.clone(),
        obj_cv: (!mutable && !is_static, false),
        ref_q: false,
    };
    cx.lambdas.push(ctx);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    crate::declarations::functions::definitions::check::define(cx, op, &d, body);
    cx.default_arg = saved_default;
    let ctx = cx.lambdas.pop().unwrap();
    // LAM-17: a constant of the enclosing function whose address the body
    // takes, or that a reference binds, needs a capture.
    if let Some(&(v, s)) = ctx.constants.first() {
        if ctx.default.is_some() {
            return unsupported(cx, s, "a use of a constant as an object in a lambda with a capture default");
        }
        cx.error_cause(s, "LAM-17", Some("LAM-9"), format!("the lambda uses the constant '{}' as an object without capturing it", cx.u.entities[v].name));
        return bad(span);
    }
    cx.u.classes[class].complete = true;
    if let Some(c) = cx.u.closures.get_mut(&class) {
        c.captures = ctx.inits.len();
    }
    let ty = QType::new(Type::Class(class));
    Expr::new(EK::List(Box::new(Init::List(ctx.inits))), ty, Cat::PR, span)
}

/// An init capture (LAM-7): a new data member that its initializer
/// initializes, found by the name of the capture in the body.
fn init_capture(cx: &mut Check, ctx: &mut LambdaCtx, w: &Written, init: &Node) -> bool {
    let Some(e) = first_node(init).filter(|x| x.prod != "braced_init" && x.prod != "expr_list") else {
        // `x{e}` and `x(e)`: one expression in the brackets.
        let inner = first_node(init).and_then(|x| {
            let xs: Vec<&Node> = x.kids.iter().filter_map(|k| match k {
                Kid::Node(c) => Some(c),
                _ => None,
            }).collect();
            (xs.len() == 1).then(|| xs[0].clone())
        });
        let Some(inner) = inner.filter(|x| x.prod != "expr_list" && x.prod != "init_list") else {
            cx.unsupported(init.span, "an init capture of this form");
            return false;
        };
        return init_capture_expr(cx, ctx, w, &inner);
    };
    init_capture_expr(cx, ctx, w, &e.clone())
}

fn init_capture_expr(cx: &mut Check, ctx: &mut LambdaCtx, w: &Written, e: &Node) -> bool {
    let v = expr(cx, e);
    if crate::expressions::primary::check::unresolved_set(&v) {
        crate::expressions::primary::check::no_target(cx, &v, "OVL-20");
        return false;
    }
    if v.ty.is_error() {
        return false;
    }
    let (ty, i) = if w.by_ref {
        // LAM-7: a temporary that the reference binds lives as long as
        // the closure, which the machine does not model.
        if v.cat != Cat::L || v.any(&|x| matches!(x.k, EK::Conv(crate::sem::tree::Conv::Materialize, _))) {
            cx.unsupported(w.span, "an init capture by reference of a temporary");
            return false;
        }
        let v = crate::foundations::values::check::deref_ref(v);
        (QType::lref(v.ty.clone()), Init::Expr(Expr::new(EK::AddrOf(Box::new(v.clone())), QType::ptr_to(v.ty.clone()), Cat::PR, v.span)))
    } else {
        let t = crate::declarations::auto::check::decay(&v.ty.nonref().clone()).unqual_top();
        if t.is_class() {
            let r = rvalue(cx, v);
            (t, Init::Expr(r))
        } else {
            match crate::declarations::initialization::forms::check::copy_init(cx, v, &t, "LAM-7") {
                Some(x) => (t, Init::Expr(x)),
                None => return false,
            }
        }
    };
    let scope = cx.u.classes[ctx.class].scope;
    let var = cx.u.add(crate::sem::Entity { name: w.name.clone(), kind: EntKind::Var(local_var(ty.clone())), span: w.span, scope, linkage: Linkage::None, deprecated: None });
    let fty = if w.by_ref { QType::ptr_to(ty.nonref().clone()) } else { ty };
    let field = new_field(cx, ctx.class, fty);
    ctx.init_vars.push((var, w.by_ref, field));
    ctx.inits.push(i);
    true
}

/// A data member of the closure type.
fn new_field(cx: &mut Check, class: ClassId, ty: QType) -> EntityId {
    let index = cx.u.classes[class].fields.len();
    let scope = cx.u.classes[class].scope;
    let f = cx.u.add(crate::sem::Entity {
        name: "".into(),
        kind: EntKind::Field(Field { ty, class, index, init: None, mutable: false, access: crate::sem::Access::Public }),
        span: Span::default(),
        scope,
        linkage: Linkage::None,
        deprecated: None,
    });
    cx.u.classes[class].fields.push(f);
    f
}

/// Capture `v` in `ctx` with the expression `e` that names it in the
/// enclosing context (LAM-4): a copy of its object, or its address.
fn add_capture(cx: &mut Check, ctx: &mut LambdaCtx, v: EntityId, by_ref: bool, e: Expr) {
    let e = crate::foundations::values::check::deref_ref(e);
    let t = e.ty.clone();
    let (fty, init) = if by_ref {
        let p = QType::ptr_to(t.clone());
        (p.clone(), Init::Expr(Expr::new(EK::AddrOf(Box::new(e.clone())), p, Cat::PR, e.span)))
    } else {
        (t.clone(), copy_of(cx, e))
    };
    let field = new_field(cx, ctx.class, fty);
    ctx.caps.push((v, by_ref, field));
    ctx.inits.push(init);
}

/// The copy of an object that an lvalue names: each element of an array
/// (LAM-4).
fn copy_of(cx: &mut Check, e: Expr) -> Init {
    match &e.ty.t {
        Type::Array(el, Some(n)) => {
            let el = (**el).clone();
            let span = e.span;
            Init::List((0..*n).map(|k| {
                let size_t = QType::new(Type::Int(cx.u.target.size_t()));
                let p = crate::foundations::conversions::check::decay(cx, e.clone());
                let x = Expr::new(EK::Index(Box::new(p), Box::new(Expr::int(k as i128, size_t, span))), el.clone(), Cat::L, span);
                copy_of(cx, x)
            }).collect())
        }
        _ => Init::Expr(rvalue(cx, e)),
    }
}

/// The lambda, innermost first, whose closure type holds the scope.
fn owner(cx: &Check, mut s: crate::sem::ScopeId) -> Option<usize> {
    loop {
        if let Some(k) = cx.lambdas.iter().rposition(|l| cx.u.classes[l.class].scope == s) {
            return Some(k);
        }
        s = cx.u.scopes[s].parent?;
    }
}

/// The member `field` of the closure of lambda `k`, seen in its body: the
/// object of a capture by copy, or the object a capture by reference
/// points to.
fn member(cx: &mut Check, k: usize, field: EntityId, by_ref: bool, span: Span) -> Expr {
    let l = &cx.lambdas[k];
    let class = l.class;
    let op = cx.u.closures.get(&class).map(|c| c.op);
    let cv = op.and_then(|o| cx.u.func(o)).map(|f| f.obj_cv).unwrap_or((false, false));
    let obj_ty = QType::new(Type::Class(class)).with_cv(cv.0, cv.1);
    let this = Expr::new(EK::This, QType::ptr_to(obj_ty.clone()), Cat::PR, span);
    let obj = Expr::new(EK::Deref(Box::new(this)), obj_ty, Cat::L, span);
    let m = crate::classes::members::check::field_of(cx, obj, field, span);
    if !by_ref {
        return m;
    }
    let p = rvalue(cx, m);
    let t = p.ty.pointee().cloned().unwrap_or_else(QType::error);
    Expr::new(EK::Deref(Box::new(p)), t, Cat::L, span)
}

/// The expression that names `v` in the body of lambda `level`, or in the
/// enclosing function for None: the variable, or the member of a capture
/// that holds it, made with the capture default where needed (LAM-5).
/// None after an error (LAM-9).
fn use_at(cx: &mut Check, level: Option<usize>, v: EntityId, span: Span) -> Option<Expr> {
    let own = owner(cx, cx.u.entities[v].scope);
    if level.is_none() || level == own {
        if let Some(k) = own {
            if let Some(&(_, by_ref, field)) = cx.lambdas[k].init_vars.iter().find(|x| x.0 == v) {
                return Some(member(cx, k, field, by_ref, span));
            }
        }
        let ty = cx.u.var(v).map(|x| x.ty.clone()).unwrap_or_else(QType::error);
        return Some(crate::foundations::values::check::deref_ref(Expr::new(EK::Var(v), ty, Cat::L, span)));
    }
    let k = level.unwrap();
    if let Some(&(_, by_ref, field)) = cx.lambdas[k].caps.iter().find(|x| x.0 == v) {
        return Some(member(cx, k, field, by_ref, span));
    }
    let Some(by_ref) = cx.lambdas[k].default else {
        let name = cx.u.entities[v].name.clone();
        cx.error(span, "LAM-9", format!("the lambda uses '{}', which it does not capture", name));
        return None;
    };
    if cx.bindings.contains(&v) {
        cx.unsupported(span, "a capture of a structured binding");
        return None;
    }
    let outer = if k == 0 { None } else { Some(k - 1) };
    let e = use_at(cx, outer, v, span)?;
    let mut ctx = cx.lambdas[k].clone();
    add_capture(cx, &mut ctx, v, by_ref, e);
    cx.lambdas[k] = ctx;
    let &(_, by_ref, field) = cx.lambdas[k].caps.last().unwrap();
    Some(member(cx, k, field, by_ref, span))
}

/// The expression that names `v` where the lambda expression stands.
fn enclosing_use(cx: &mut Check, v: EntityId, span: Span) -> Option<Expr> {
    let top = cx.lambdas.len().checked_sub(1);
    use_at(cx, top, v, span)
}

/// A variable that a read of its value can use without a capture
/// (LAM-9, ODR-3): a variable usable in constant expressions.
/// A variable with a `mutable` member at any depth is not one.
fn is_constant(cx: &Check, v: EntityId) -> bool {
    cx.u.var(v).is_some_and(|x| !x.param && x.const_init && (x.ty.is_ref() || x.constexpr || (x.ty.c && !x.ty.v && (x.ty.is_integral() || x.ty.is_enum()))) && !has_mutable(cx, &x.ty))
}

fn has_mutable(cx: &Check, t: &QType) -> bool {
    t.nonref().base_elem().class().is_some_and(|c| {
        cx.u.classes[c].fields.iter().any(|&f| cx.u.field(f).is_some_and(|x| x.mutable || has_mutable(cx, &x.ty)))
    })
}

/// A use of the variable `v` inside a lambda body: the member of the
/// capture that holds it (LAM-5, LAM-9). None when the use needs no
/// capture, and Err after an error.
pub fn capture_check(cx: &mut Check, v: EntityId, span: Span) -> Option<Result<Expr, ()>> {
    if cx.lambdas.is_empty() || !in_lambda_body(cx) {
        return None;
    }
    let var = cx.u.var(v)?;
    if var.storage != Storage::Automatic || !cx.u.is_local(cx.u.entities[v].scope) && owner(cx, cx.u.entities[v].scope).is_none() {
        return None;
    }
    let top = cx.lambdas.len() - 1;
    let own = owner(cx, cx.u.entities[v].scope);
    if own == Some(top) && !cx.lambdas[top].init_vars.iter().any(|x| x.0 == v) {
        return None;
    }
    let held = cx.lambdas[top].caps.iter().any(|x| x.0 == v) || cx.lambdas[top].init_vars.iter().any(|x| x.0 == v);
    if !held && (cx.unevaluated > 0 || is_constant(cx, v)) {
        // A constant read as a value needs no capture; the read resolves
        // the use (`read`). A reference that a constant expression
        // initializes is no use (ODR-3).
        let is_ref = cx.u.var(v).is_some_and(|x| x.ty.is_ref());
        if cx.unevaluated == 0 && !is_ref {
            cx.lambdas[top].constants.push((v, span));
        }
        return None;
    }
    Some(use_at(cx, Some(top), v, span).ok_or(()))
}

/// ODR-3: a read of a scalar value, or an expression whose value is
/// discarded, is no use of the constants among its potential results:
/// through parentheses, a member, an element, the operands of `?:`, and
/// the right operand of a comma. Such a constant of an enclosing
/// function needs no capture (LAM-9); the machine reads it from its
/// initializer.
pub fn read(cx: &mut Check, e: &Expr) {
    if cx.lambdas.last().is_none_or(|l| l.constants.is_empty()) {
        return;
    }
    match &e.k {
        EK::Var(v) => {
            let l = cx.lambdas.last_mut().unwrap();
            if let Some(pos) = l.constants.iter().position(|&(x, s)| x == *v && s == e.span) {
                l.constants.remove(pos);
            }
        }
        EK::Member(x, _) | EK::Conv(crate::sem::tree::Conv::NoOp | crate::sem::tree::Conv::ArrayDecay, x) | EK::Comma(_, x) => read(cx, x),
        EK::Index(a, _) => read(cx, a),
        EK::Cond(_, a, b) => {
            read(cx, a);
            read(cx, b);
        }
        _ => {}
    }
}

/// LAM-11: a copy of a closure without captures reads no object, so a
/// copy of a variable that holds one is the empty closure.
pub fn empty_copy(cx: &Check, e: &Expr) -> Option<Expr> {
    let c = e.ty.class()?;
    let cl = cx.u.closures.get(&c)?;
    (cl.captures == 0 && matches!(e.k, EK::Var(_))).then(|| Expr::new(EK::List(Box::new(Init::List(Vec::new()))), e.ty.unqual(), Cat::PR, e.span))
}

/// LAM-23: a modification of a capture by copy in a lambda that is not
/// mutable.
pub fn copy_capture(cx: &Check, l: &Expr) -> bool {
    let EK::Member(obj, _) = &l.k else { return false };
    let EK::Deref(t) = &obj.k else { return false };
    matches!(t.k, EK::This) && obj.ty.c && obj.ty.class().is_some_and(|c| cx.u.closures.contains_key(&c))
}

/// LAM-10: the conversion of a closure without captures to a pointer to
/// a function of the call operator's type. Err with the reason when the
/// closure has captures (LAM-25) or the type differs.
pub fn to_pointer(cx: &Check, e: &Expr, to: Option<&QType>) -> Option<Result<Expr, String>> {
    let c = e.ty.nonref().class()?;
    let cl = cx.u.closures.get(&c)?;
    let f = cx.u.func(cl.op)?;
    let own = QType::ptr_to(QType::new(Type::Func(f.ty.clone())));
    if cl.captures > 0 {
        return Some(Err(format!("a lambda with captures does not convert to a pointer to function (LAM-25)")));
    }
    let to = to.cloned().unwrap_or_else(|| own.clone());
    let ok = match (to.pointee().and_then(|p| p.func()), f.ty.as_ref()) {
        (Some(t), g) => t.params == g.params && t.ret == g.ret && t.variadic == g.variadic && (g.noexcept || !t.noexcept),
        _ => false,
    };
    if !ok {
        return Some(Err(format!("no implicit conversion from {} to {}", e.ty.show(), to.show())));
    }
    Some(Ok(Expr::new(EK::Closure(cl.op), to.unqual(), Cat::PR, e.span)))
}

/// A call of a closure object (LAM-1): its call operator, with the object
/// as the implicit object argument. None when `obj` is not a closure.
pub fn call_operator(cx: &Check, obj: &Expr) -> Option<EntityId> {
    let c = obj.ty.nonref().class()?;
    cx.u.closures.get(&c).map(|cl| cl.op)
}

/// The closure type of a class with captures, which has no default
/// constructor and no copy assignment (LAM-26).
pub fn has_captures(cx: &Check, c: ClassId) -> bool {
    cx.u.closures.get(&c).is_some_and(|cl| cl.captures > 0)
}

/// LAM-22: the returns of a lambda without a trailing return type give
/// one type; the rule of a function with a deduced return type otherwise.
pub fn return_rule(cx: &Check) -> (&'static str, Option<&'static str>) {
    if in_lambda_body(cx) {
        ("LAM-22", Some("RET-7"))
    } else {
        ("RET-7", None)
    }
}
