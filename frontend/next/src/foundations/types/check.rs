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

/// The type of a name used as a type specifier.
pub fn named_type(cx: &mut Check, n: &Node) -> QType {
    let mut qual = names::check::Qual::None;
    let mut name = None;
    for k in &n.kids {
        match k {
            Kid::Node(c) if c.prod == "nested_name" => qual = names::check::nested_name(cx, c),
            Kid::Node(c) => return cx.unsupported(c.span, format!("a type named by {}", c.prod)),
            _ => name = Some((cx.text(k), cx.token(k).map(|t| t.span).unwrap_or(n.span))),
        }
    }
    let Some((name, span)) = name else { return cx.unsupported(n.span, "a type name") };
    let found = match qual {
        names::check::Qual::None => names::check::lookup(cx, name),
        names::check::Qual::Scope(s) => names::check::lookup_in(cx, s, name),
        _ => return QType::error(),
    };
    for e in found {
        crate::declarations::attributes::check::use_entity(cx, e, span);
        match &cx.u.entities[e].kind {
            EntKind::Typedef(t) => return t.clone(),
            EntKind::Enum(en) => return QType::new(Type::Enum(*en)),
            EntKind::Class(c) => return QType::new(Type::Class(*c)),
            EntKind::Unsupported(_) => return cx.unsupported(span, format!("the type '{}', which the front end does not model", name)),
            _ => {}
        }
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
            "class_def" => {
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
