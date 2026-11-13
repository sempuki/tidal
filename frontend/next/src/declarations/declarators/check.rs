//! The check part of `declarations/declarators`: the type a declarator
//! gives to the name it declares, and the rules on compound types (DCL,
//! CV).

use crate::declarations::auto::check::Placeholder;
use crate::declarations::check::Specs;
use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::names::check::{self as names, Qual};
use crate::sem::check::Check;
use crate::sem::types::{FnType, QType, Type};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// A parameter of a function declarator.
#[derive(Clone, Debug)]
pub struct Param {
    pub name: Option<(Rc<str>, Span)>,
    pub ty: QType,
    /// The top-level cv-qualifiers as written, which the parameter keeps
    /// inside the body though the function type drops them (DCL-9).
    pub cv: (bool, bool),
    /// The parameter is declared with an array type, which DCL-9
    /// adjusts to a pointer (UN-19).
    pub was_array: bool,
    pub default: Option<Node>,
    pub span: Span,
    pub maybe_unused: bool,
}

/// What a declarator declares.
#[derive(Clone, Debug)]
pub struct Declared {
    pub name: Option<(Rc<str>, Span)>,
    /// The scope of a qualified declarator-id.
    pub qual: Qual,
    pub ty: QType,
    /// The parameters of the function declarator applied to the name.
    pub params: Vec<Param>,
    pub span: Span,
    pub unsupported: bool,
    /// The placeholder of the specifiers, still to deduce.
    pub placeholder: Option<Placeholder>,
    /// A function with a trailing return type.
    pub trailing: bool,
    /// The attributes after the declarator-id.
    pub attrs: crate::declarations::attributes::check::Attrs,
    /// The declarator node, which a placeholder's deduction reads again.
    pub node: Node,
    /// The cv-qualifiers and ref-qualifier after the parameters of the
    /// function declarator applied to the name: the object of a member
    /// function.
    pub obj_cv: (bool, bool),
    pub ref_q: bool,
}

/// The declarator of a declaration with these specifiers.
pub fn declarator(cx: &mut Check, specs: &Specs, n: &Node) -> Declared {
    let mut d = Declared {
        name: None,
        qual: Qual::None,
        ty: QType::error(),
        params: Vec::new(),
        span: n.span,
        unsupported: false,
        placeholder: specs.placeholder.clone(),
        trailing: false,
        attrs: Default::default(),
        node: n.clone(),
        obj_cv: (false, false),
        ref_q: false,
    };
    let base = match (&specs.ty, &specs.placeholder) {
        (Some(t), _) => t.clone(),
        (None, Some(_)) => QType::error(),
        (None, None) => {
            if !specs.unsupported {
                d.unsupported = true;
                cx.unsupported(n.span, "a declaration with no type specifier");
            }
            QType::error()
        }
    };
    let saved = std::mem::replace(&mut cx.naming_type, specs.typedef);
    let t = apply(cx, base, n, &mut d, true);
    cx.naming_type = saved;
    d.ty = t;
    d
}

/// The declarator applied again to a deduced type (`declarations/auto`).
pub fn with_base(cx: &mut Check, base: QType, n: &Node) -> QType {
    let mut d = Declared {
        name: None,
        qual: Qual::None,
        ty: QType::error(),
        params: Vec::new(),
        span: n.span,
        unsupported: false,
        placeholder: None,
        trailing: false,
        attrs: Default::default(),
        node: n.clone(),
        obj_cv: (false, false),
        ref_q: false,
    };
    let marks = cx.u.diags.len();
    let t = apply(cx, base, n, &mut d, false);
    cx.u.diags.truncate(marks);
    t
}

/// The type of a `type_id`: type specifiers and an abstract declarator.
pub fn type_id(cx: &mut Check, n: &Node) -> QType {
    let specs: Vec<&Node> = n.child("type_specifiers").map(|s| s.children("type_specifier").collect()).unwrap_or_default();
    let (ty, ph, _, unsup) = crate::foundations::types::check::type_specifiers(cx, &specs, n.span);
    if unsup {
        return QType::error();
    }
    let Some(base) = ty else {
        if ph.is_some() {
            return cx.unsupported(n.span, "a placeholder in a type-id");
        }
        return cx.unsupported(n.span, "a type-id with no type");
    };
    match n.child("abstract_declarator") {
        Some(a) => {
            let mut d = Declared {
                name: None,
                qual: Qual::None,
                ty: QType::error(),
                params: Vec::new(),
                span: n.span,
                unsupported: false,
                placeholder: None,
                trailing: false,
                attrs: Default::default(),
                node: a.clone(),
                obj_cv: (false, false),
                ref_q: false,
            };
            apply(cx, base, a, &mut d, false)
        }
        None => base,
    }
}

/// The type of the name inside `n`, when the part of the declaration
/// outside `n` gives `t`. `top` marks the declarator whose function
/// suffix gives the parameters of the declared function.
fn apply(cx: &mut Check, t: QType, n: &Node, d: &mut Declared, top: bool) -> QType {
    match n.prod {
        "declarator" | "abstract_declarator" => {
            let mut t = t;
            let mut inner = None;
            for k in &n.kids {
                match k {
                    Kid::Node(c) if c.prod == "ptr_part" => t = ptr_part(cx, t, c),
                    Kid::Node(c) => inner = Some(c),
                    _ => {
                        if cx.text(k) == "..." {
                            d.unsupported = true;
                            cx.unsupported(n.span, "a pack in a declarator");
                        }
                    }
                }
            }
            match inner {
                Some(c) => apply(cx, t, c, d, top),
                None => t,
            }
        }
        "direct_declarator" | "abstract_direct" => {
            let kids: Vec<&Kid> = n.kids.iter().collect();
            // `( declarator )`
            if matches!(kids.first(), Some(k @ Kid::Tok(_)) if cx.text(k) == "(") {
                if let Some(Kid::Node(inner)) = kids.get(1) {
                    let mut t = t;
                    // A suffix after the parenthesized part applies first.
                    for k in kids.iter().skip(3) {
                        if let Kid::Node(s) = k {
                            t = suffix(cx, t, s, d, false);
                        }
                    }
                    return apply(cx, t, inner, d, top);
                }
            }
            let mut t = t;
            let mut inner: Option<&Node> = None;
            let mut suffixes = Vec::new();
            for k in &n.kids {
                match k {
                    Kid::Node(c) if c.prod == "declarator_id" => declarator_id(cx, c, d),
                    Kid::Node(c) if c.prod == "attribute_spec" => crate::declarations::attributes::check::attribute_spec(cx, c, &mut d.attrs),
                    Kid::Node(c) if c.prod == "direct_declarator" || c.prod == "abstract_direct" => inner = Some(c),
                    Kid::Node(c) if c.prod == "array_suffix" || c.prod == "function_suffix" => suffixes.push(c),
                    Kid::Node(c) => {
                        d.unsupported = true;
                        cx.unsupported(c.span, format!("a declarator part of the form {}", c.prod));
                    }
                    _ => {
                        if cx.text(k) == "..." {
                            d.unsupported = true;
                            cx.unsupported(n.span, "a pack in a declarator");
                        }
                    }
                }
            }
            let direct = inner.is_some_and(|i| i.child("declarator_id").is_some() && i.kids.len() == 1) || inner.is_none();
            // NAM-21: after a qualified declarator-id, the rest of the
            // declarator looks names up in the scope that it names.
            let saved = cx.scope;
            if let Some(nn) = n.find("declarator_id").and_then(|i| i.find("nested_name")) {
                let marks = cx.u.diags.len();
                if let Qual::Scope(q) = names::nested_name(cx, nn) {
                    cx.scope = q;
                }
                // The declarator-id reports its own faults.
                cx.u.diags.truncate(marks);
            }
            for s in suffixes.iter().rev() {
                t = suffix(cx, t, s, d, top && direct);
            }
            cx.scope = saved;
            match inner {
                Some(i) => apply(cx, t, i, d, top),
                None => t,
            }
        }
        _ => {
            d.unsupported = true;
            cx.unsupported(n.span, format!("a declarator of the form {}", n.prod));
            QType::error()
        }
    }
}

fn declarator_id(cx: &mut Check, n: &Node, d: &mut Declared) {
    // A destructor `~C`, which the class's members name as such (DTOR-1).
    if cx.member_of.is_some() && cx.has_word(n, "~") {
        if let Some(c) = n.child("class_name") {
            d.name = Some((Rc::from(format!("~{}", cx.source(c)).as_str()), c.span));
            return;
        }
    }
    // An operator function (OPR-1).
    if let Some(o) = n.child("operator_function_id") {
        if let Some(nn) = n.child("nested_name") {
            d.qual = names::nested_name(cx, nn);
            if d.qual == Qual::Unsupported {
                d.unsupported = true;
                return;
            }
        }
        match crate::classes::operators::check::function_name(cx, o) {
            Some(name) => d.name = Some((Rc::from(name), o.span)),
            None => {
                d.unsupported = true;
                cx.unsupported(n.span, "an operator function of this form");
            }
        }
        return;
    }
    let Some(id) = n.child("id_expression") else {
        d.unsupported = true;
        cx.unsupported(n.span, "a conversion function, a literal operator, or a declarator-id of this form");
        return;
    };
    match id.kids.first() {
        Some(Kid::Node(u)) if u.prod == "unqualified_id" => match u.kids.as_slice() {
            [k @ Kid::Tok(_)] => d.name = Some((Rc::from(cx.text(k)), cx.token(k).map(|t| t.span).unwrap_or(n.span))),
            _ => {
                d.unsupported = true;
                cx.unsupported(u.span, "a declarator-id that is not an identifier");
            }
        },
        Some(Kid::Node(q)) if q.prod == "qualified_name" => {
            if let Some(nn) = q.child("nested_name") {
                d.qual = names::nested_name(cx, nn);
            }
            match q.child("unqualified_id") {
                Some(u) if d.qual != Qual::Unsupported && matches!(u.kids.as_slice(), [Kid::Node(o)] if o.prod == "operator_function_id") => match crate::classes::operators::check::unqualified_name(cx, u) {
                    Some(name) => d.name = Some((Rc::from(name), u.span)),
                    None => d.unsupported = true,
                },
                Some(u) => match u.kids.as_slice() {
                    [k @ Kid::Tok(_)] if d.qual != Qual::Unsupported => d.name = Some((Rc::from(cx.text(k)), cx.token(k).map(|t| t.span).unwrap_or(n.span))),
                    _ => d.unsupported = true,
                },
                None => d.unsupported = true,
            }
        }
        _ => {
            d.unsupported = true;
            cx.unsupported(n.span, "a declarator-id of this form");
        }
    }
}

/// `ptr_part`: a pointer with its cv-qualifiers, or a reference.
/// A function type with a cv-qualifier or a ref-qualifier (DCL-16).
pub fn qualified_func(t: &QType) -> bool {
    matches!(&t.t, Type::Func(f) if f.qualified)
}

fn ptr_part(cx: &mut Check, t: QType, n: &Node) -> QType {
    let words = cx.words(n);
    if n.child("nested_name").is_some() {
        return cx.unsupported(n.span, "a pointer to member");
    }
    let (mut c, mut v) = (0, 0);
    for k in n.children("cv") {
        match cx.first_word(k) {
            "const" => c += 1,
            _ => v += 1,
        }
    }
    if c > 1 || v > 1 {
        duplicate_cv(cx, n.span);
    }
    if qualified_func(&t) && matches!(words.first().copied(), Some("*" | "&" | "&&")) {
        return cx.error(n.span, "DCL-16", "a pointer or a reference to a function type with a cv-qualifier or a ref-qualifier");
    }
    match words.first().copied() {
        Some("*") => {
            if t.is_ref() {
                return cx.error(n.span, "DCL-17", "a pointer to a reference");
            }
            QType::ptr_to(t).with_cv(c > 0, v > 0)
        }
        Some(r @ ("&" | "&&")) => {
            if c + v > 0 {
                return cx.error(n.span, "DCL-18", "a cv-qualifier applied to a reference");
            }
            if t.is_ref() {
                return cx.error(n.span, "DCL-18", "a reference to a reference");
            }
            if t.is_void() {
                return cx.error(n.span, "DCL-18", "a reference to void");
            }
            QType::new(Type::Ref(Rc::new(t), r == "&&"))
        }
        _ => cx.unsupported(n.span, "a pointer declarator of this form"),
    }
}

fn suffix(cx: &mut Check, t: QType, s: &Node, d: &mut Declared, top: bool) -> QType {
    match s.prod {
        "array_suffix" => array_suffix(cx, t, s),
        _ => function_suffix(cx, t, s, d, top),
    }
}

/// `array_suffix` (DCL-19, DCL-20).
fn array_suffix(cx: &mut Check, t: QType, n: &Node) -> QType {
    if t.is_error() {
        return t;
    }
    if t.is_ref() {
        return cx.error(n.span, "DCL-18", "an array of references");
    }
    if t.is_func() || t.is_void() {
        return cx.error(n.span, "DCL-20", format!("an array of {}", if t.is_void() { "void" } else { "functions" }));
    }
    if let Type::Array(_, None) = t.t {
        return cx.error(n.span, "DCL-20", "an array of arrays omits a bound other than its first");
    }
    let bound = match n.kids.iter().find_map(|k| match k {
        Kid::Node(c) if c.prod != "attribute_spec" => Some(c),
        _ => None,
    }) {
        None => None,
        Some(e) => match ce::array_bound(cx, e) {
            Some(b) => Some(b),
            None => return QType::error(),
        },
    };
    // DCL-19, STG-1: the bound and the size of the array in bytes fit in
    // std::size_t.
    if let Some(b) = bound {
        let max = cx.u.target.int_max(cx.u.target.size_t()) as u128;
        let bytes = cx.u.target.size_of(&cx.u, &t).map(|s| s as u128 * b as u128);
        if b as u128 > max || bytes.is_some_and(|s| s > max) {
            return cx.error(n.span, "DCL-19", "the size of the array does not fit in std::size_t");
        }
    }
    QType::new(Type::Array(Rc::new(t), bound))
}

/// A declarator whose name is qualified by a class: an out-of-class
/// declaration of a member.
fn names_class_member(cx: &Check, n: &Node) -> bool {
    let nn = match n.find("qualified_name") {
        Some(q) => q.child("nested_name"),
        None => n.find("declarator_id").and_then(|d| d.child("nested_name")),
    };
    let Some(nn) = nn else { return false };
    let mut toks = Vec::new();
    nn.tokens(&mut toks);
    let Some(name) = toks.iter().rev().map(|&t| cx.toks[t].text.as_str()).find(|w| *w != "::") else { return false };
    !crate::foundations::names::check::lookup_where(cx, name, |k| matches!(k, crate::sem::EntKind::Class(_))).is_empty()
}

/// `function_suffix`: the parameters, the cv-qualifiers, the exception
/// specification, and a trailing return type (DCL-21, DCL-22).
fn function_suffix(cx: &mut Check, ret: QType, n: &Node, d: &mut Declared, top: bool) -> QType {
    let mut ret = ret;
    // DCL-15: an alias or a typedef can name a qualified function type.
    let qualified = cx.naming_type && (n.children("cv").next().is_some() || n.child("ref_qualifier").is_some());
    // A member function: declared in its class, or named with its class
    // as the qualifier. Its qualifiers belong to its object.
    let member = (top && (cx.member_of.is_some() || names_class_member(cx, &d.node))) || d.node.find("ptr_part").is_some_and(|p| p.find("nested_name").is_some());
    if member {
        for c in n.children("cv") {
            match cx.first_word(c) {
                "const" => d.obj_cv.0 = true,
                _ => d.obj_cv.1 = true,
            }
        }
        d.ref_q = n.child("ref_qualifier").is_some();
    }
    let not_member = !qualified && !member;
    if not_member && n.children("cv").next().is_some() {
        cx.error(n.span, "DCL-21", "a function that is not a member has a cv-qualifier");
    }
    if not_member && n.child("ref_qualifier").is_some() {
        cx.error(n.span, "DCL-21", "a function that is not a member has a ref-qualifier");
    }
    let (params, variadic) = parameters(cx, n.child("parameters"));
    let mut noexcept = false;
    if let Some(e) = n.child("exception_spec") {
        noexcept = crate::declarations::exception_specifications::check::exception_spec(cx, e, &params);
    }
    if let Some(tr) = n.child("type_id") {
        let rt = type_id(cx, tr);
        if d.placeholder.is_none() || !ret.is_error() {
            cx.error(n.span, "DCL-21", "a trailing return type needs 'auto' at the start");
        }
        if top {
            d.trailing = true;
            d.placeholder = None;
        }
        ret = rt;
    }
    crate::declarations::volatile::check::top_level(cx, &ret, n.span, "return type");
    // DEFARG-5: only the declaration of a function has default
    // arguments, not a function type that an alias names, nor a pointer
    // or a reference to a function.
    if (!top || cx.naming_type) && params.iter().any(|p| p.default.is_some()) {
        cx.error(n.span, "DEFARG-5", "a default argument outside the declaration of a function");
    }
    if ret.is_array() {
        cx.error(n.span, "DCL-21", "a function returns an array");
    } else if ret.is_func() {
        cx.error(n.span, "DCL-21", "a function returns a function");
    }
    let f = FnType { ret, params: params.iter().map(|p| p.ty.clone()).collect(), variadic, noexcept, qualified };
    if top {
        d.params = params;
    }
    QType::new(Type::Func(Rc::new(f)))
}

/// `parameters`: each parameter's type after the adjustments of DCL-9:
/// an array becomes a pointer, a function becomes a pointer to it, and a
/// top-level cv-qualifier goes.
pub fn parameters(cx: &mut Check, n: Option<&Node>) -> (Vec<Param>, bool) {
    let Some(n) = n else { return (Vec::new(), false) };
    let words = cx.words(n);
    let variadic = words.contains(&"...");
    let ps: Vec<&Node> = n.children("parameter").collect();
    if words == ["void"] {
        return (Vec::new(), false);
    }
    let mut out = Vec::new();
    for p in &ps {
        // CLS-14: an explicit object parameter belongs to a member
        // function, which build step 4 part a does not model.
        if p.kids.iter().any(|k| cx.text(k) == "this") {
            cx.unsupported(p.span, "an explicit object parameter (member functions come with build step 4)");
            return (Vec::new(), variadic);
        }
        let mut attrs = crate::declarations::attributes::check::Attrs::default();
        for a in p.children("attribute_spec") {
            crate::declarations::attributes::check::attribute_spec(cx, a, &mut attrs);
        }
        let Some(ds) = p.child("decl_specifiers") else { continue };
        let specs = crate::declarations::check::decl_specifiers(cx, ds, attrs.clone());
        if specs.is_static || specs.is_extern || specs.thread_local || specs.mutable {
            crate::declarations::storage_linkage::check::parameter_storage(cx, ds.span);
        }
        if specs.inline {
            crate::declarations::inline::check::misplaced(cx, ds.span, "on a parameter");
        }
        if specs.constexpr || specs.consteval || specs.constinit {
            crate::declarations::constexpr::check::on_parameter(cx, ds.span);
        }
        if specs.placeholder.is_some() {
            cx.unsupported(p.span, "a placeholder in a parameter (an abbreviated function template)");
        }
        let mut d = Declared {
            name: None,
            qual: Qual::None,
            ty: QType::error(),
            params: Vec::new(),
            span: p.span,
            unsupported: false,
            placeholder: None,
            trailing: false,
            attrs: Default::default(),
            node: (*p).clone(),
            obj_cv: (false, false),
            ref_q: false,
        };
        let base = specs.ty.clone().unwrap_or_else(QType::error);
        let decl = p.child("declarator").or_else(|| p.child("abstract_declarator"));
        let mut t = match decl {
            Some(dn) => apply(cx, base, dn, &mut d, false),
            None => base,
        };
        if t.is_void() {
            if ps.len() > 1 || d.name.is_some() || t.c || t.v {
                cx.error(p.span, "DCL-22", "void in a parameter list");
            } else {
                cx.error(p.span, "DCL-22", "void in a parameter list in a form other than (void)");
            }
            t = QType::error();
        }
        if qualified_func(&t) {
            cx.error(p.span, "DCL-16", "a parameter of a function type with a cv-qualifier or a ref-qualifier");
            t = QType::error();
        }
        crate::declarations::volatile::check::top_level(cx, &t, p.span, "parameter type");
        let cv = if t.is_array() || t.is_func() { (false, false) } else { (t.c, t.v) };
        // FN-13: two parameters of one declaration with the same name.
        if let Some((n, sp)) = &d.name {
            if out.iter().any(|q: &Param| q.name.as_ref().is_some_and(|(m, _)| m == n)) {
                cx.error(*sp, "FN-13", format!("the parameter name '{}' occurs twice", n));
            }
        }
        let was_array = t.is_array();
        t = adjust_parameter(t);
        let default = p.kids.iter().skip_while(|k| !matches!(k, Kid::Tok(_)) || cx.text(k) != "=").nth(1).and_then(|k| match k {
            Kid::Node(e) => Some(e.clone()),
            _ => None,
        });
        out.push(Param { name: d.name.clone(), ty: t, cv, was_array, default, span: p.span, maybe_unused: attrs.maybe_unused });
    }
    (out, variadic)
}

/// DCL-9's adjustments of a parameter type.
pub fn adjust_parameter(t: QType) -> QType {
    match &t.t {
        Type::Array(e, _) => QType::ptr_to((**e).clone()),
        Type::Func(_) => QType::ptr_to(t.clone()),
        _ => t.unqual_top(),
    }
}

/// CV-2: the same cv-qualifier twice directly in one type.
pub fn duplicate_cv(cx: &mut Check, span: Span) {
    cx.error(span, "CV-2", "duplicate cv-qualifier");
}

/// CV-3: a `const` scalar variable that is defined needs an initializer.
pub fn const_needs_init(cx: &mut Check, span: Span, name: &str) {
    cx.error(span, "CV-3", format!("the const variable '{}' has no initializer", name));
}

/// DCL-18: a reference variable needs an initializer.
pub fn reference_needs_init(cx: &mut Check, span: Span, name: &str) {
    cx.error(span, "DCL-18", format!("the reference '{}' has no initializer", name));
}

/// CV-4: a modification through an lvalue of `const` type. `context`
/// is the rule of the operator, given as the cause.
pub fn modify_const(cx: &mut Check, span: Span, what: &str, context: &'static str) {
    cx.error_cause(span, "CV-4", Some(context), format!("{} of a const object", what));
}

impl QType {
    /// The type without its top-level cv-qualifiers.
    pub fn unqual_top(&self) -> QType {
        match &self.t {
            Type::Array(..) => self.clone(),
            _ => QType::new(self.t.clone()),
        }
    }
}

/// DCL-16: a function type with a cv-qualifier or a ref-qualifier is not
/// the type of a variable. Build step 3 has no such function types.
pub fn is_qualified_function_type(_t: &QType) -> bool {
    false
}
