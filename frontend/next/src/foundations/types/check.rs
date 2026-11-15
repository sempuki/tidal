//! The check part of `foundations/types`: the type that a list of type
//! specifiers names (TYP-6, TYP-7), and the queries on types that the
//! rules of this page give.

use crate::declarations::auto::check::Placeholder;
use crate::foundations::names;
use crate::sem::check::Check;
use crate::sem::types::{FloatK, IntK, QType, Type};
use crate::sem::EntKind;
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The type that the keywords of fundamental types give (TYP-6), or a
/// TYP-7 error for a combination a program must not write.
pub fn keywords(cx: &mut Check, kw: &[&str], span: Span) -> QType {
    let count = |s: &str| kw.iter().filter(|k| **k == s).count();
    let (signed, unsigned, short, long, int) = (count("signed"), count("unsigned"), count("short"), count("long"), count("int"));
    let others: Vec<&&str> = kw.iter().filter(|k| !matches!(**k, "signed" | "unsigned" | "short" | "long" | "int")).collect();
    let bad = |cx: &mut Check| cx.error(span, "TYP-7", format!("invalid combination of type specifiers: {}", kw.join(" ")));
    if signed + unsigned > 1 || (short > 0 && long > 0) || long > 2 || short > 1 || int > 1 || others.len() > 1 {
        return bad(cx);
    }
    if let Some(o) = others.first() {
        let t = match **o {
            "char" => {
                if short + long + int > 0 {
                    return bad(cx);
                }
                Type::Int(if signed > 0 {
                    IntK::SChar
                } else if unsigned > 0 {
                    IntK::UChar
                } else {
                    IntK::Char
                })
            }
            "double" if long == 1 && signed + unsigned + short + int == 0 => Type::Float(FloatK::LongDouble),
            o => {
                if signed + unsigned + short + long + int > 0 {
                    return bad(cx);
                }
                match o {
                    "void" => Type::Void,
                    "bool" => Type::Int(IntK::Bool),
                    "char8_t" => Type::Int(IntK::Char8),
                    "char16_t" => Type::Int(IntK::Char16),
                    "char32_t" => Type::Int(IntK::Char32),
                    "wchar_t" => Type::Int(IntK::WChar),
                    "float" => Type::Float(FloatK::Float),
                    "double" => Type::Float(FloatK::Double),
                    _ => return cx.unsupported(span, format!("the type specifier '{}'", o)),
                }
            }
        };
        return QType::new(t);
    }
    QType::new(Type::Int(match (unsigned > 0, short, long) {
        (false, 1, _) => IntK::Short,
        (true, 1, _) => IntK::UShort,
        (false, 0, 1) => IntK::Long,
        (true, 0, 1) => IntK::ULong,
        (false, 0, 2) => IntK::LongLong,
        (true, 0, 2) => IntK::ULongLong,
        (true, _, _) => IntK::UInt,
        _ => IntK::Int,
    }))
}

/// ACC-2, ACC-5: the member `e` is accessible from the class `x`: a
/// public member, a member of `x` or of a class that encloses `x`, a
/// protected member of a base of `x`, or a member of a class that
/// befriends `x`.
fn accessible_from_class(cx: &Check, e: crate::sem::EntityId, x: crate::sem::types::ClassId) -> bool {
    let access = cx.u.access.get(&e).copied().unwrap_or(crate::sem::Access::Public);
    if access == crate::sem::Access::Public {
        return true;
    }
    let Some(owner) = cx.u.class_of_scope(cx.u.entities[e].scope) else { return true };
    let mut s = Some(cx.u.classes[x].scope);
    while let Some(sc) = s {
        if sc == cx.u.classes[owner].scope {
            return true;
        }
        s = cx.u.scopes[sc].parent;
    }
    if access == crate::sem::Access::Protected && crate::classes::derived::check::derives(cx, x, owner) {
        return true;
    }
    cx.u.friend_classes.get(&owner).is_some_and(|fs| fs.contains(&x))
}

/// The type of a name used as a type specifier.
pub fn named_type(cx: &mut Check, n: &Node) -> QType {
    let mut qual = names::check::Qual::None;
    let mut name = None;
    for k in &n.kids {
        match k {
            Kid::Node(c) if c.prod == "nested_name" => qual = names::check::nested_name(cx, c),
            // DCLT rules: `decltype(e)` names the type of `e`, as in a
            // functional cast.
            Kid::Node(c) if matches!(c.prod, "decltype_expr" | "decltype_spec") && n.kids.len() == 1 => return crate::expressions::decltype::check::decltype_spec(cx, c),
            // TARG-1: a specialization of a class template.
            Kid::Node(c) if c.prod == "template_id" && n.kids.len() == 1 => {
                if let Some(t) = crate::templates::declarations::check::class_template_use(cx, c) {
                    return t;
                }
                return cx.unsupported(c.span, format!("a type named by {}", c.prod));
            }
            Kid::Node(c) => return cx.unsupported(c.span, format!("a type named by {}", c.prod)),
            _ => name = Some((cx.text(k), cx.token(k).map(|t| t.span).unwrap_or(n.span))),
        }
    }
    let Some((name, span)) = name else { return cx.unsupported(n.span, "a type name") };
    // NAM-34: a class's name after its own qualifier names its
    // constructor, not the class.
    if let names::check::Qual::Scope(s) = qual {
        if cx.u.scopes[s].kind == crate::sem::ScopeKind::Class && &*cx.u.scopes[s].name == name {
            return cx.error(span, "NAM-34", format!("'{}::{}' names the constructor, not a type", name, name));
        }
    }
    let found = match qual {
        names::check::Qual::None => names::check::lookup(cx, name),
        names::check::Qual::Scope(s) => names::check::lookup_in(cx, s, name),
        _ => return QType::error(),
    };
    if matches!(qual, names::check::Qual::None) && !crate::classes::derived::check::injected_accessible(cx, name) {
        return cx.error(span, "ACC-14", format!("the name '{}' of a base is inaccessible here", name));
    }
    for e in found {
        crate::declarations::attributes::check::use_entity(cx, e, span);
        // ACC-2: a member type is named where its access allows. Outside a
        // function body, a class body and the declaration of a member of a
        // class check access as in that class (ACC-11).
        // ACC-9: a using-declaration in the qualifier's class gives the
        // name the access of the using-declaration.
        let through_using = match qual {
            names::check::Qual::Scope(s) => cx.u.class_of_scope(s).and_then(|k| cx.u.using_access.get(&(k, e)).copied()) == Some(crate::sem::Access::Public),
            _ => false,
        };
        if !through_using && cx.u.access.contains_key(&e) && matches!(cx.u.entities[e].kind, EntKind::Typedef(_) | EntKind::Enum(_) | EntKind::Class(_)) && !crate::classes::members::check::accessible(cx, e) {
            let from = if cx.in_function() { None } else { cx.access_class.or(cx.member_of) };
            if !from.is_some_and(|x| accessible_from_class(cx, e, x)) {
                return cx.error(span, "ACC-2", format!("the member type '{}' is not accessible here", name));
            }
        }
        match &cx.u.entities[e].kind {
            EntKind::Typedef(t) => return t.clone(),
            EntKind::Enum(en) => return QType::new(Type::Enum(*en)),
            EntKind::Class(c) => return QType::new(Type::Class(*c)),
            EntKind::Unsupported(_) => return cx.unsupported(span, format!("the type '{}', which the front end does not model", name)),
            _ => {}
        }
    }
    // DER-4, NAM-36: the injected names of two bases make the name
    // ambiguous in the class.
    let class = cx.member_of.or_else(|| cx.func.as_ref().and_then(|f| f.id).and_then(|id| cx.u.func(id)).and_then(|f| f.class));
    if matches!(qual, names::check::Qual::None) && class.is_some_and(|c| crate::classes::derived::check::member_lookup(cx, c, name).is_err()) {
        cx.error_cause(span, "DER-4", Some("NAM-36"), format!("the name '{}' is in more than one base", name));
        return QType::error();
    }
    cx.unsupported(span, format!("the type name '{}'", name))
}

/// The type the type specifiers of a declaration give, the placeholder
/// if any, whether they define a type, and whether a part is
/// unsupported.
pub fn type_specifiers(cx: &mut Check, specs: &[&Node], span: Span) -> (Option<QType>, Option<Placeholder>, bool, bool) {
    let mut kw: Vec<&str> = Vec::new();
    let (mut c, mut v) = (0, 0);
    let mut ty: Option<QType> = None;
    let mut ph = None;
    let mut defines = false;
    let mut unsup = false;
    for s in specs {
        let Some(Kid::Node(x)) = s.kids.first() else { continue };
        match x.prod {
            "cv" => match cx.first_word(x) {
                "const" => c += 1,
                _ => v += 1,
            },
            "simple_type" => {
                if x.kids.len() == 1 && matches!(x.kids[0], Kid::Tok(_)) && is_fundamental(cx.text(&x.kids[0])) {
                    kw.push(cx.text(&x.kids[0]));
                } else {
                    ty = Some(named_type(cx, x));
                }
            }
            "placeholder" => ph = Some(crate::declarations::auto::check::placeholder(cx, x)),
            "enum_def" => {
                ty = Some(crate::declarations::enumerations::check::enum_def(cx, x));
                defines = true;
            }
            "decltype_spec" | "decltype_expr" => ty = Some(crate::expressions::decltype::check::decltype_spec(cx, x)),
            "elaborated_type" => {
                ty = Some(crate::declarations::enumerations::check::elaborated_type(cx, x));
                // `struct S;` declares the class.
                defines = cx.first_word(x) != "enum";
            }
            "class_def" | "union_def" => {
                ty = Some(crate::classes::members::check::class_def(cx, x));
                defines = true;
            }
            _ => {
                unsup = true;
                cx.unsupported(x.span, format!("a type specifier of the form {}", x.prod));
                ty = Some(QType::error());
            }
        }
    }
    if !kw.is_empty() {
        if ty.is_some() {
            cx.error(span, "TYP-7", "a fundamental type keyword with a type name");
        }
        ty = Some(keywords(cx, &kw, span));
    }
    if c > 1 || v > 1 {
        crate::declarations::declarators::check::duplicate_cv(cx, span);
    }
    if let Some(t) = &mut ty {
        *t = t.with_cv(c > 0, v > 0);
    }
    if let Some(p) = &mut ph {
        p.c = c > 0;
        p.v = v > 0;
    }
    (ty, ph, defines, unsup)
}

pub fn is_fundamental(w: &str) -> bool {
    matches!(w, "void" | "bool" | "char" | "char8_t" | "char16_t" | "char32_t" | "wchar_t" | "short" | "int" | "long" | "signed" | "unsigned" | "float" | "double")
}

/// A declaration with specifiers and no declarator must declare a type
/// (`declarations/type-names`).
pub fn declares_nothing(cx: &mut Check, span: Span) {
    crate::foundations::translation::check::declares_nothing(cx, span);
}

/// A complete object type: not `void`, not an array of unknown bound,
/// not a function or reference.
pub fn is_complete_object(q: &QType) -> bool {
    !q.is_incomplete() && !q.is_func() && !q.is_ref()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sem::Unit;

    #[test]
    fn keyword_combinations_follow_typ_6_and_typ_7() {
        let mut cx = Check::new(&[], Unit::default());
        let sp = Span::default();
        assert_eq!(keywords(&mut cx, &["unsigned", "long", "long", "int"], sp).t, Type::Int(IntK::ULongLong));
        assert_eq!(keywords(&mut cx, &["long", "double"], sp).t, Type::Float(FloatK::LongDouble));
        assert_eq!(keywords(&mut cx, &["signed", "char"], sp).t, Type::Int(IntK::SChar));
        assert_eq!(cx.errors(), 0);
        keywords(&mut cx, &["long", "short"], sp);
        keywords(&mut cx, &["unsigned", "bool"], sp);
        assert_eq!(cx.errors(), 2);
        assert_eq!(cx.u.diags[0].rule, Some("TYP-7"));
    }
}
