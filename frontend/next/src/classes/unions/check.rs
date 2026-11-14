//! The check part of `classes/unions`: the limits on the members of a
//! union (UNI-2, UNI-8) and the members that the rewrite models. A union
//! whose member has a non-trivial special member function (UNI-3,
//! UNI-11) and an anonymous union (UNI-7, UNI-9) are unsupported. The
//! page's bit-fields are here too (BF rules).

use crate::expressions::check::{expr, nodes};
use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::names::check as names;
use crate::foundations::values::check::rvalue;
use crate::machine::Val;
use crate::sem::check::Check;
use crate::sem::tree::Init;
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{ClassKey, EntKind, EntityId, Field, Linkage, Unit};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// Whether class `c` is a union.
pub fn is_union(cx: &Check, c: ClassId) -> bool {
    cx.u.classes[c].key == ClassKey::Union
}

/// Whether the special member functions of a type are all trivial and
/// not declared by the program: no constructor, destructor, or
/// assignment that the program provides, no virtual function, and no
/// default member initializer, in the type and in its members.
fn plain(u: &Unit, ty: &QType) -> bool {
    match &ty.t {
        Type::Array(e, _) => plain(u, e),
        Type::Class(c) => {
            let names = &u.scopes[u.classes[*c].scope].names;
            let provided = |n: &str| names.get(n).is_some_and(|fs| fs.iter().any(|f| !u.defaulted.contains(f)));
            let ctor = crate::classes::constructors::check::CTOR;
            let dtor = crate::classes::constructors::check::DTOR;
            !u.polymorphic(*c) && !provided(ctor) && !provided(dtor) && !provided("operator=") && u.classes[*c].fields.iter().all(|&f| u.field(f).is_some_and(|x| x.init.is_none() && plain(u, &x.ty)))
        }
        _ => true,
    }
}

/// A non-static data member of the union `c`: no reference (UNI-2). True
/// when the member is valid; an unsupported member gives false after its
/// diagnostic too.
pub fn member(cx: &mut Check, c: ClassId, ty: &QType, name: &str, span: Span) -> Option<bool> {
    if !is_union(cx, c) {
        return Some(true);
    }
    if ty.is_ref() {
        cx.error(span, "UNI-2", format!("the union member '{}' has a reference type", name));
        return Some(false);
    }
    if !plain(&cx.u, ty) {
        cx.unsupported(span, "a union member of a class with special member functions that are not trivial (build step 4)");
        return None;
    }
    Some(true)
}

/// UNI-8: at most one variant member has a default member initializer.
pub fn end_of_union(cx: &mut Check, c: ClassId, span: Span) {
    if !is_union(cx, c) {
        return;
    }
    let n = cx.u.classes[c].fields.iter().filter(|&&f| cx.u.field(f).is_some_and(|x| x.init.is_some())).count();
    if n > 1 {
        cx.error(span, "UNI-8", "two members of a union have default member initializers");
    }
}

/// The initializer with each inactive member of a union in it (a member
/// whose initializer is default-initialization, UNI-4) taken as having a
/// value, for the checks of a constant initializer (CE-21).
pub fn without_inactive(cx: &Check, ty: &QType, init: &Init) -> Init {
    match (init, &ty.t) {
        (Init::List(xs), Type::Class(c)) => {
            let union = is_union(cx, *c);
            let tys = cx.u.field_types(*c);
            Init::List(xs.iter().enumerate().map(|(k, x)| match (x, tys.get(k)) {
                (Init::Default, _) if union => Init::Zero,
                (_, Some(t)) => without_inactive(cx, t, x),
                _ => x.clone(),
            }).collect())
        }
        (Init::List(xs), Type::Array(e, _)) => Init::List(xs.iter().map(|x| without_inactive(cx, e, x)).collect()),
        _ => init.clone(),
    }
}

/// A `bitfield` member declarator of class `c` (BF-1, BF-2, BF-8): a
/// named or unnamed bit-field of integral or enumeration type, whose
/// width is a constant of integral or unscoped enumeration type. The
/// field and its default member initializer go to the class. True when
/// the declaration is valid; None after an unsupported form.
pub fn bitfield<'n>(cx: &mut Check, c: ClassId, specs: &crate::declarations::check::Specs, n: &'n Node, access: crate::sem::Access, inits: &mut Vec<(EntityId, &'n Node)>) -> Option<bool> {
    let span = n.span;
    let name: String = match n.kids.first() {
        Some(k @ Kid::Tok(_)) if cx.text(k) != ":" => cx.text(k).to_string(),
        _ => String::new(),
    };
    if specs.typedef || specs.is_static {
        cx.error(span, "BF-2", "a bit-field that is static or a typedef");
        return Some(false);
    }
    let Some(ty) = specs.ty.clone() else {
        cx.error(span, "BF-2", "a bit-field with a placeholder type");
        return Some(false);
    };
    if ty.is_error() {
        return Some(false);
    }
    if !(ty.is_integral() || ty.is_enum()) {
        cx.error(span, "BF-2", format!("the bit-field has type {}, which is not integral or an enumeration", ty.show()));
        return Some(false);
    }
    if name.is_empty() && (ty.c || ty.v) {
        cx.error(span, "BF-2", "an unnamed bit-field with a cv-qualifier");
        return Some(false);
    }
    let w = nodes(n).into_iter().find(|x| x.prod != "member_init")?;
    let e = expr(cx, w);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return Some(false);
    }
    if e.ty.is_class() {
        cx.unsupported(e.span, "a bit-field width of class type");
        return None;
    }
    let unscoped = matches!(e.ty.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !(e.ty.is_integral() || unscoped) {
        cx.error(e.span, "BF-2", format!("the width of a bit-field has type {}", e.ty.show()));
        return Some(false);
    }
    let width = match ce::try_value(cx, &e) {
        Some(Val::Int(v)) => v,
        _ => {
            cx.error(e.span, "BF-2", "the width of a bit-field is not a constant");
            return Some(false);
        }
    };
    if width < 0 {
        cx.error(e.span, "BF-2", "the width of a bit-field is negative");
        return Some(false);
    }
    if width == 0 && !name.is_empty() {
        cx.error(span, "BF-2", format!("the bit-field '{}' has width 0", name));
        return Some(false);
    }
    // BF-8: a width larger than the type.
    let bits = type_bits(&cx.u, &ty);
    if width as u64 > bits && !name.is_empty() {
        cx.warning(span, "BF-8", format!("the width of the bit-field '{}' exceeds its type", name));
    }
    let index = cx.u.classes[c].fields.len();
    let f = Field { ty, class: c, index, init: None, mutable: specs.mutable, access, base: false, width: Some(width as u64) };
    let fid = if name.is_empty() {
        let scope = cx.u.classes[c].scope;
        cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Field(f), span, scope, linkage: Linkage::None, deprecated: None })
    } else {
        names::declare_in(cx, cx.scope, &name, EntKind::Field(f), span, Linkage::None)
    };
    cx.u.classes[c].fields.push(fid);
    if let Some(i) = n.child("member_init") {
        inits.push((fid, i));
    }
    Some(true)
}

/// The number of value bits of an integral or enumeration type.
pub fn type_bits(u: &Unit, ty: &QType) -> u64 {
    match &ty.t {
        Type::Int(k) => u.target.int_bits(*k) as u64,
        Type::Enum(e) => u.enums.get(*e).and_then(|i| i.underlying).map(|k| u.target.int_bits(k) as u64).unwrap_or(32),
        _ => 0,
    }
}

/// The width of the bit-field that an expression names, if any: a
/// member access, also after the lvalue-to-rvalue conversion.
pub fn width_of(u: &Unit, e: &crate::sem::tree::Expr) -> Option<u64> {
    use crate::sem::tree::{Conv, EK};
    match &e.k {
        EK::Member(x, i) => x.ty.class().and_then(|c| u.classes[c].fields.get(*i).copied()).and_then(|f| u.field(f)).and_then(|f| f.width),
        EK::Conv(Conv::LToR | Conv::NoOp, x) => width_of(u, x),
        // The result of an assignment to a bit-field is the bit-field
        // (ASN-10), and so is an lvalue result of `?:` or `,` that
        // refers to one (COND-9).
        EK::Assign(l, _) | EK::Compound(_, l, _, _) | EK::IncDec(true, _, l) if e.cat != crate::sem::tree::Cat::PR => width_of(u, l),
        EK::Cond(_, a, b) if e.cat != crate::sem::tree::Cat::PR => width_of(u, a).or_else(|| width_of(u, b)),
        EK::Comma(_, b) if e.cat != crate::sem::tree::Cat::PR => width_of(u, b),
        _ => None,
    }
}

/// An unnamed bit-field, which is no member (BF-12).
pub fn is_unnamed_bitfield(u: &Unit, f: EntityId) -> bool {
    u.field(f).is_some_and(|x| x.width.is_some()) && u.entities[f].name.is_empty()
}

/// BF-8: a constant stored in the bit-field `l` changes value because of
/// the width.
pub fn truncation_warning(cx: &mut Check, l: &crate::sem::tree::Expr, r: &crate::sem::tree::Expr, span: Span) {
    let Some(w) = width_of(&cx.u, l) else { return };
    warn_if_truncated(cx, w, &l.ty, r, span);
}

/// BF-8 for the value `r` stored in a bit-field of width `w` and type
/// `ty`.
pub fn warn_if_truncated(cx: &mut Check, w: u64, ty: &QType, r: &crate::sem::tree::Expr, span: Span) {
    if ty.is_bool() || !ty.is_integral() {
        return;
    }
    let Some(Val::Int(v)) = ce::try_value(cx, r) else { return };
    let bits = type_bits(&cx.u, ty);
    let n = w.min(bits).min(127) as u32;
    let signed = ty.int_kind().is_some_and(|k| k.signed());
    let fits = if n == 0 { v == 0 } else if signed { v >= -(1i128 << (n - 1)) && v < (1i128 << (n - 1)) } else { v >= 0 && v < (1i128 << n) };
    if !fits {
        cx.warning(span, "BF-8", format!("the value {} changes when it is stored in a bit-field of width {}", v, w));
    }
}

/// UNI-7, UNI-9: the union `u` of an anonymous union. Its members must be
/// public non-static data members, with no nested type and no name that
/// the enclosing scope has; they become names of the enclosing scope.
/// True when the union is valid.
pub fn anonymous(cx: &mut Check, u: ClassId, span: Span) -> bool {
    let scope = cx.u.classes[u].scope;
    let mut members: Vec<(std::rc::Rc<str>, EntityId)> = cx.u.scopes[scope].names.iter().flat_map(|(n, es)| es.iter().map(move |&e| (n.clone(), e))).collect();
    members.sort_by_key(|x| x.1);
    for (n, e) in &members {
        let bad = match &cx.u.entities[*e].kind {
            EntKind::Field(x) => x.access != crate::sem::Access::Public,
            _ => true,
        };
        if bad {
            cx.error(span, "UNI-9", format!("the anonymous union has the member '{}', which is not a public non-static data member", n));
            return false;
        }
    }
    // An unnamed class member, such as a nested anonymous union, and a
    // class that the union defines, also an unnamed one.
    let nested = cx.u.classes.iter().any(|k| cx.u.scopes[k.scope].parent == Some(scope));
    if nested || cx.u.classes[u].fields.iter().any(|&f| cx.u.entities[f].name.is_empty() && cx.u.field(f).is_some_and(|x| x.width.is_none())) {
        cx.error(span, "UNI-9", "an anonymous union with a nested type or an unnamed class member");
        return false;
    }
    // CLS-9: in a class, no member has the class's name.
    if let Some(k) = cx.u.class_of_scope(cx.scope) {
        if members.iter().any(|(n, _)| **n == *cx.u.classes[k].name) {
            cx.error(span, "CLS-9", format!("a member of an anonymous union has the name of the class '{}'", cx.u.classes[k].name));
            return false;
        }
    }
    for (n, _) in &members {
        if !names::declared_here(cx, cx.scope, n).is_empty() {
            cx.error(span, "UNI-9", format!("a member of the anonymous union has the name '{}', which the enclosing scope has", n));
            return false;
        }
    }
    let here = cx.scope;
    for (n, e) in members {
        cx.u.scopes[here].names.entry(n).or_default().push(e);
    }
    true
}

/// The position of the unnamed member of class `k` that holds the
/// anonymous union `u` (UNI-7).
pub fn holder(u: &Unit, k: ClassId, anon: ClassId) -> Option<usize> {
    u.classes[k].fields.iter().position(|&f| u.entities[f].name.is_empty() && u.field(f).is_some_and(|x| !x.base && x.width.is_none() && x.ty.class() == Some(anon)))
}

/// The `union_def` that is a type specifier of these decl-specifiers
/// itself, not one inside a class that they define.
pub fn own_union(ds: &Node) -> Option<&Node> {
    fn walk(n: &Node) -> Option<&Node> {
        if n.prod == "union_def" {
            return Some(n);
        }
        if n.prod == "class_def" {
            return None;
        }
        n.kids.iter().find_map(|k| match k {
            Kid::Node(c) => walk(c),
            _ => None,
        })
    }
    walk(ds)
}

/// A name of a member of an anonymous union member of class `c`: the
/// position of the holding member and of the union's member (UNI-7).
pub fn variant_member(u: &Unit, c: ClassId, name: &str) -> Option<(usize, usize)> {
    u.classes[c].fields.iter().enumerate().find_map(|(k, &f)| {
        let anon = u.field(f).and_then(|x| x.ty.class()).filter(|&a| u.entities[f].name.is_empty() && holder(u, c, a) == Some(k))?;
        let j = u.classes[anon].fields.iter().position(|&g| &*u.entities[g].name == name)?;
        Some((k, j))
    })
}

/// The initializer of an anonymous union member with its member `j`
/// active.
pub fn variant_init(u: &Unit, c: ClassId, k: usize, j: usize, i: Init) -> Init {
    let n = u.field(u.classes[c].fields[k]).and_then(|x| x.ty.class()).map(|a| u.classes[a].fields.len()).unwrap_or(0);
    let mut i = Some(i);
    Init::List((0..n).map(|x| if x == j { i.take().unwrap_or(Init::Default) } else { Init::Default }).collect())
}
