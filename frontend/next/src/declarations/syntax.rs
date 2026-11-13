//! The Syntax segment of `declarations/index`: the start symbol
//! `translation_unit`, `declaration`, `block_decl`, `simple_decl`,
//! `init_declarator`, and the specifiers.

use crate::classes::{members, unions};
use crate::declarations::alignment::syntax::alignment_specifier;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::auto::syntax::placeholder;
use crate::declarations::declarators::syntax::{cv, declarator, declares_function, function_suffix_of};
use crate::declarations::enumerations::syntax::{enum_def, enum_opaque, using_enum_decl};
use crate::declarations::functions::definitions::syntax::function_body;
use crate::declarations::initialization::forms::syntax::init;
use crate::declarations::linkage_specifications::syntax::linkage_spec;
use crate::declarations::namespaces::syntax::{namespace_alias, namespace_def, using_decl, using_directive};
use crate::declarations::static_assert::syntax::static_assert_decl;
use crate::declarations::storage_linkage::syntax::storage_class;
use crate::declarations::structured_bindings::syntax::sb_decl;
use crate::declarations::type_names::syntax::{alias_decl, elaborated_type};
use crate::expressions::decltype::syntax::decltype_spec;
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::classes::constructors::syntax::explicit_spec;
use crate::classes::r#virtual::syntax::virt_specifiers;
use crate::lex::TK;
use crate::syntax::names::{DeclCtx, Enter, Kind, Qual, TKind};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;
use crate::templates::class_template_deduction::syntax::deduction_guide;
use crate::templates::concepts::syntax::{concept_definition, requires_clause};
use crate::templates::declarations::syntax::template_decl;
use crate::templates::dependent_names::syntax::typename_specifier;
use crate::templates::instantiation::syntax::explicit_instantiation;
use crate::templates::specialization::syntax::explicit_specialization;

/// `translation_unit: {declaration}`.
pub fn translation_unit(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while !p.at_eof() {
        let mut d = declaration(p)?;
        p.fill(&mut d);
        kids.push(Kid::Node(d));
    }
    Ok(p.node("translation_unit", sp, kids))
}

/// `declaration`: a block declaration, a function definition, a
/// namespace, a linkage specification, a template, a specialization, an
/// instantiation, a deduction guide, a concept, or `;`.
pub fn declaration(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("declaration") {
        return Ok(n);
    }
    let sp = p.span();
    let k = if p.at(";") {
        p.bump()
    } else if p.at("namespace") && p.ahead(2) != "=" || p.at("inline") && p.ahead(1) == "namespace" {
        Kid::Node(namespace_def(p)?)
    } else if p.at("extern") && p.ahead_kind(1) == TK::Str {
        Kid::Node(linkage_spec(p)?)
    } else if p.at("extern") && p.ahead(1) == "template" || p.at("template") && p.ahead(1) != "<" {
        Kid::Node(explicit_instantiation(p)?)
    } else if p.at("template") && p.ahead(2) == ">" {
        Kid::Node(explicit_specialization(p)?)
    } else if p.at("template") {
        match p.attempt(concept_definition) {
            Ok(n) => Kid::Node(n),
            Err(_) => Kid::Node(template_decl(p)?),
        }
    } else if looks_like_guide(p) {
        Kid::Node(deduction_guide(p)?)
    } else {
        Kid::Node(decl_or_definition(p)?)
    };
    Ok(p.node("declaration", sp, vec![k]))
}

/// A deduction guide starts with a class template's name, its
/// parameters in parentheses, and `->`.
pub fn looks_like_guide(p: &Parser) -> bool {
    let k = if p.at("explicit") {
        if p.ahead(1) == "(" {
            return false;
        }
        1
    } else {
        0
    };
    if p.ahead_kind(k) != TK::Ident || p.ahead(k + 1) != "(" {
        return false;
    }
    if !matches!(p.names.find(Qual::None, p.ahead(k)).kind, Kind::Template(TKind::Class)) {
        return false;
    }
    let mut depth = 0;
    let mut j = k + 1;
    loop {
        match p.ahead(j) {
            "(" => depth += 1,
            ")" => {
                depth -= 1;
                if depth == 0 {
                    return p.ahead(j + 1) == "->";
                }
            }
            _ if p.ahead_kind(j) == TK::Eof => return false,
            _ => {}
        }
        j += 1;
    }
}

/// A block declaration or a function definition, which share their
/// start: attributes, specifiers, and a declarator.
fn decl_or_definition(p: &mut Parser) -> PResult<Node> {
    if let Some(b) = p.opt(block_decl_keyword) {
        return Ok(b);
    }
    let sp = p.span();
    let mut pre = Vec::new();
    while p.at_attr() || p.at("alignas") {
        pre.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
    }
    let specs = p.opt(specs_here);
    if specs.is_none() && pre.iter().all(|k| matches!(k, Kid::Node(n) if n.prod == "attribute_spec")) {
        // A function definition may have no specifiers (a constructor).
        let n = function_definition_after(p, sp, pre, None)?;
        return Ok(n);
    }
    let Some(specs) = specs else { return Err(p.fail("declaration specifiers")) };
    if p.at(";") {
        let mut kids = pre;
        kids.push(Kid::Node(specs));
        kids.push(p.bump());
        let s = p.node("simple_decl", sp, kids);
        return Ok(p.node("block_decl", sp, vec![Kid::Node(s)]));
    }
    let m = p.mark();
    let d = declarator_here(p)?;
    if let Some(f) = p.opt(|p| function_definition_rest(p, sp, pre.clone(), Some(specs.clone()), d.clone())) {
        return Ok(f);
    }
    p.reset(m);
    let s = simple_decl_after(p, sp, pre, specs)?;
    Ok(p.node("block_decl", sp, vec![Kid::Node(s)]))
}

/// The block declarations that a keyword begins, and the structured
/// binding and the opaque enumeration, which need a whole-form try.
fn block_decl_keyword(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let a = after_attrs(p);
    let k = if p.at("using") || p.at("static_assert") || p.at("namespace") || (a > 0 && p.ahead(a) == "using" && p.ahead(a + 1) == "namespace") {
        return block_decl(p);
    } else if p.at("enum") {
        enum_opaque(p)?
    } else if looks_like_sb(p) {
        sb_decl(p)?
    } else {
        return Err(p.fail("a declaration"));
    };
    Ok(p.node("block_decl", sp, vec![Kid::Node(k)]))
}

/// The number of tokens of the attribute specifiers that start here.
pub fn after_attrs(p: &Parser) -> usize {
    let mut k = 0;
    while p.ahead(k) == "[" && p.ahead(k + 1) == "[" {
        let mut depth = 0i32;
        loop {
            match p.ahead(k) {
                "[" => depth += 1,
                "]" => depth -= 1,
                "" if p.ahead_kind(k) == TK::Eof => return k,
                _ => {}
            }
            k += 1;
            if depth == 0 {
                break;
            }
        }
    }
    k
}

/// The start of a structured binding: `auto`, an optional `&` or `&&`,
/// and `[`.
pub fn looks_like_sb(p: &Parser) -> bool {
    let mut k = after_attrs(p);
    while matches!(p.ahead(k), "static" | "thread_local" | "const" | "volatile") {
        k += 1;
    }
    if p.ahead(k) != "auto" {
        return false;
    }
    k += 1;
    if matches!(p.ahead(k), "&" | "&&") {
        k += 1;
    }
    p.ahead(k) == "["
}

/// FN-9: a function definition in a block, `T name(...) {...}`. The
/// other readings take the body as a braced initializer and stop after
/// it, so the rule's stop goes after the body.
fn nested_definition(p: &mut Parser) {
    let mut k = 0;
    while k < 16 && !matches!(p.ahead(k), "(" | ";" | "=" | "{" | "") {
        k += 1;
    }
    if p.ahead(k) != "(" || k == 0 || p.ahead_kind(k - 1) != TK::Ident {
        return;
    }
    let close = |p: &Parser, mut k: usize, open: &str, shut: &str| -> Option<usize> {
        let mut depth = 0;
        loop {
            match p.ahead(k) {
                "" if p.ahead_kind(k) == TK::Eof => return None,
                s if s == open => depth += 1,
                s if s == shut => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(k + 1);
                    }
                }
                _ => {}
            }
            k += 1;
        }
    };
    let Some(mut j) = close(p, k, "(", ")") else { return };
    while matches!(p.ahead(j), "const" | "noexcept" | "override" | "final") {
        j += 1;
    }
    if p.ahead(j) != "{" {
        return;
    }
    let Some(end) = close(p, j, "{", "}") else { return };
    let m = p.mark();
    for _ in 0..end {
        p.bump();
    }
    p.fail_rule("FN-9", "a function definition in a block");
    p.reset(m);
}

/// `block_decl: simple_decl | sb_decl | alias_decl | enum_opaque
/// | static_assert_decl | using_decl | using_enum_decl | using_directive
/// | namespace_alias`.
pub fn block_decl(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("block_decl") {
        return Ok(n);
    }
    let sp = p.span();
    let a = after_attrs(p);
    let k = if a > 0 && p.ahead(a) == "using" && p.ahead(a + 1) == "namespace" {
        using_directive(p)?
    } else if p.at("using") {
        if p.ahead(1) == "namespace" {
            using_directive(p)?
        } else if p.ahead(1) == "enum" {
            using_enum_decl(p)?
        } else if p.ahead_kind(1) == TK::Ident && matches!(p.ahead(2), "=" | "[" | "alignas") {
            alias_decl(p)?
        } else {
            using_decl(p)?
        }
    } else if p.at("static_assert") {
        static_assert_decl(p)?
    } else if p.at("extern") && p.ahead_kind(1) == TK::Str {
        p.bump();
        return Err(p.fail_rule("LSP-2", "a linkage specification in a block"));
    } else if p.at("namespace") && p.ahead(1) != "=" && p.ahead(2) != "=" {
        // NS-10: a namespace definition in a block. The stop goes at the
        // `{`, where the reading as an alias stops.
        let m = p.mark();
        while !p.at("{") && !p.at(";") && !p.at_eof() {
            p.bump();
        }
        let f = p.fail_rule("NS-10", "a namespace definition in a block");
        p.reset(m);
        return Err(f);
    } else if p.at("namespace") {
        namespace_alias(p)?
    } else if p.at("enum") {
        match p.attempt(enum_opaque) {
            Ok(n) => n,
            Err(_) => simple_decl(p)?,
        }
    } else if looks_like_sb(p) {
        sb_decl(p)?
    } else {
        nested_definition(p);
        simple_decl(p)?
    };
    Ok(p.node("block_decl", sp, vec![Kid::Node(k)]))
}

/// `simple_decl: {attribute_spec | alignment_specifier} decl_specifiers
/// [init_declarator {"," init_declarator}] ";"`.
pub fn simple_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut pre = Vec::new();
    while p.at_attr() || p.at("alignas") {
        pre.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
    }
    let specs = specs_here(p)?;
    simple_decl_after(p, sp, pre, specs)
}

fn simple_decl_after(p: &mut Parser, sp: crate::source::Span, pre: Vec<Kid>, specs: Node) -> PResult<Node> {
    let tmpl = p.take_template();
    let mut decls = Vec::new();
    if !p.at(";") {
        kids_push_declarators(p, &mut decls, &specs, tmpl)?;
    }
    let mut kids = pre;
    kids.push(Kid::Node(specs));
    kids.extend(decls);
    kids.push(p.expect(";")?);
    Ok(p.node("simple_decl", sp, kids))
}

/// The specifiers of a declaration. Outside a block they are a position
/// of DEP-20.
pub fn specs_here(p: &mut Parser) -> PResult<Node> {
    let t = !p.names.block_scope();
    p.with(p.gt_ok, t, decl_specifiers)
}

fn kids_push_declarators(p: &mut Parser, kids: &mut Vec<Kid>, specs: &Node, tmpl: bool) -> PResult<()> {
    let mut first = true;
    loop {
        let d = init_declarator(p, specs, first && tmpl)?;
        kids.push(Kid::Node(d));
        first = false;
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => return Ok(()),
        }
    }
}

/// A declarator, read where the declaration's specifiers are known.
pub fn declarator_here(p: &mut Parser) -> PResult<Node> {
    let t = !p.names.block_scope();
    p.with(p.gt_ok, t, declarator)
}

/// `init_declarator: declarator [init] | declarator requires_clause`.
/// The name is declared before its initializer.
pub fn init_declarator(p: &mut Parser, specs: &Node, template: bool) -> PResult<Node> {
    let sp = p.span();
    let d = declarator_here(p)?;
    p.declared(&d, DeclCtx { specs: Some(specs), template, param: false });
    let mut kids = vec![Kid::Node(d)];
    if p.at("requires") {
        kids.push(Kid::Node(requires_clause(p)?));
    } else if p.at_any(&["=", "(", "{"]) {
        kids.push(Kid::Node(init(p)?));
    }
    Ok(p.node("init_declarator", sp, kids))
}

/// The rest of a `function_definition` after its declarator:
/// `[virt_specifiers] function_body` or `requires_clause function_body`.
pub fn function_definition_rest(p: &mut Parser, sp: crate::source::Span, pre: Vec<Kid>, specs: Option<Node>, d: Node) -> PResult<Node> {
    if !declares_function(&d) {
        return Err(p.fail("a function body"));
    }
    let qual = p.decl_qual;
    let mut tail = Vec::new();
    if let Some(v) = p.opt(virt_specifiers) {
        tail.push(Kid::Node(v));
    } else if p.at("requires") {
        // Names after a qualified declarator-id are looked up in the
        // scope that the qualifier names.
        if let Qual::Scope(s) = qual {
            p.names.enter(Enter::Scope(s));
        }
        let r = requires_clause(p);
        if let Qual::Scope(_) = qual {
            p.names.leave();
        }
        tail.push(Kid::Node(r?));
    }
    if !(p.at("{") || p.at("try") || p.at(":") || (p.at("=") && matches!(p.ahead(1), "delete" | "default"))) {
        return Err(p.fail("a function body"));
    }
    let tmpl = p.take_template();
    p.declared(&d, DeclCtx { specs: specs.as_ref(), template: tmpl, param: false });
    p.decl_qual = qual;
    let body = if p.class_depth > 0 && p.names.class_name().is_some() && !p.at("=") {
        p.defer_body(&d)?
    } else {
        Kid::Node(body_now(p, &d, qual)?)
    };
    let mut kids = pre;
    if let Some(s) = specs {
        kids.push(Kid::Node(s));
    }
    kids.push(Kid::Node(d));
    kids.extend(tail);
    kids.push(body);
    Ok(p.node("function_definition", sp, kids))
}

pub fn function_definition_after(p: &mut Parser, sp: crate::source::Span, pre: Vec<Kid>, specs: Option<Node>) -> PResult<Node> {
    let d = declarator_here(p)?;
    function_definition_rest(p, sp, pre, specs, d)
}

/// Parse a function body now, with the parameters declared in a block
/// scope inside the scope that the declarator's qualifier names.
pub fn body_now(p: &mut Parser, d: &Node, qual: Qual) -> PResult<Node> {
    let mut opened = 0;
    if let Qual::Scope(s) = qual {
        p.names.enter(Enter::Scope(s));
        opened += 1;
    }
    p.names.enter(Enter::Block);
    opened += 1;
    if let Some(fs) = function_suffix_of(d) {
        if let Some(ps) = fs.child("parameters") {
            for prm in ps.children("parameter") {
                p.declared(prm, DeclCtx::default());
            }
        }
    }
    let r = function_body(p);
    for _ in 0..opened {
        p.names.leave();
    }
    r
}

/// The state of a run of specifiers: whether a type name or a keyword of
/// a fundamental type came already. A second type name ends the run, so
/// that in `T x` the `x` is the declarator.
#[derive(Default, Clone, Copy)]
pub struct Specs {
    pub named: bool,
    pub builtin: bool,
}

/// `decl_specifiers: decl_specifier {decl_specifier} {attribute_spec}`.
/// An attribute after the specifiers applies to the type (ATTR-19).
pub fn decl_specifiers(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut st = Specs::default();
    let mut kids = Vec::new();
    crate::declarations::structured_bindings::syntax::near_miss(p);
    let mut k = 0;
    while matches!(p.ahead(k), "static" | "extern" | "const" | "volatile" | "constexpr" | "inline" | "thread_local") {
        k += 1;
    }
    if p.ahead(k) == "auto" && crate::foundations::types::check::is_fundamental(p.ahead(k + 1)) {
        let m = p.mark();
        for _ in 0..=k {
            p.bump();
        }
        p.fail_rule("AUTO-8", "'auto' with another type specifier");
        p.reset(m);
    }
    let saved = std::mem::replace(&mut p.ctor_check, true);
    while let Some(n) = p.opt(|p| decl_specifier(p, &mut st)) {
        kids.push(Kid::Node(n));
    }
    p.ctor_check = saved;
    if kids.is_empty() {
        return Err(p.fail("declaration specifiers"));
    }
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    Ok(p.node("decl_specifiers", sp, kids))
}

/// `decl_specifier: type_specifier | storage_class | "typedef" | "friend"
/// | "inline" | "virtual" | explicit_spec | "constexpr" | "consteval" | "constinit"`.
pub fn decl_specifier(p: &mut Parser, st: &mut Specs) -> PResult<Node> {
    let sp = p.span();
    let k = if p.at_any(&["typedef", "friend", "inline", "virtual", "constexpr", "consteval", "constinit"]) {
        p.bump()
    } else if p.at("explicit") {
        Kid::Node(explicit_spec(p)?)
    } else if let Some(s) = p.opt(storage_class) {
        Kid::Node(s)
    } else {
        Kid::Node(type_specifier(p, st)?)
    };
    Ok(p.node("decl_specifier", sp, vec![k]))
}

/// `type_specifiers: type_specifier {type_specifier}`.
pub fn type_specifiers(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut st = Specs::default();
    let mut kids = Vec::new();
    let saved = std::mem::replace(&mut p.ctor_check, false);
    while let Some(n) = p.opt(|p| type_specifier(p, &mut st)) {
        kids.push(Kid::Node(n));
    }
    p.ctor_check = saved;
    if kids.is_empty() {
        return Err(p.fail("a type"));
    }
    Ok(p.node("type_specifiers", sp, kids))
}

const FUNDAMENTAL: &[&str] = &[
    "signed", "unsigned", "short", "long", "int", "char", "wchar_t", "char8_t", "char16_t", "char32_t", "bool", "float",
    "double", "void",
];

/// `type_specifier: simple_type | cv | elaborated_type | class_def
/// | union_def | enum_def | placeholder | decltype_spec | typename_specifier`.
pub fn type_specifier(p: &mut Parser, st: &mut Specs) -> PResult<Node> {
    let sp = p.span();
    let wrap = |p: &mut Parser, n: Node| Ok(p.node("type_specifier", sp, vec![Kid::Node(n)]));
    if p.at("const") || p.at("volatile") {
        let c = cv(p)?;
        return wrap(p, c);
    }
    if p.at_any(FUNDAMENTAL) {
        if st.named {
            return Err(p.fail("a declarator"));
        }
        st.builtin = true;
        let s = simple_type(p)?;
        return wrap(p, s);
    }
    if st.named || st.builtin {
        return Err(p.fail("a declarator"));
    }
    let n = if p.at("class") || p.at("struct") {
        match p.attempt(members::syntax::class_def) {
            Ok(n) => n,
            Err(_) => elaborated_type(p)?,
        }
    } else if p.at("union") {
        match p.attempt(unions::syntax::union_def) {
            Ok(n) => n,
            Err(_) => elaborated_type(p)?,
        }
    } else if p.at("enum") {
        match p.attempt(enum_def) {
            Ok(n) => n,
            Err(_) => elaborated_type(p)?,
        }
    } else if p.at("auto") || (p.at("decltype") && p.ahead(2) == "auto" && p.ahead(3) == ")") {
        placeholder(p)?
    } else if p.at("decltype") {
        match p.attempt(simple_type) {
            Ok(n) => n,
            Err(_) => decltype_spec(p)?,
        }
    } else if p.at("typename") {
        typename_specifier(p)?
    } else if is_concept_here(p) {
        placeholder(p)?
    } else {
        simple_type(p)?
    };
    st.named = true;
    wrap(p, n)
}

/// Whether a (qualified) concept name starts here, as in `C auto`.
fn is_concept_here(p: &mut Parser) -> bool {
    let m = p.mark();
    let mut q = Qual::None;
    if p.at("::") || p.ahead(1) == "::" {
        match nested_name(p) {
            Ok((_, qq, _)) => q = qq,
            Err(_) => {
                p.reset(m);
                return false;
            }
        }
    }
    let r = p.kind() == TK::Ident && p.find(q, p.peek()).kind == Kind::Template(TKind::Concept);
    p.reset(m);
    r
}

/// `simple_type`: a keyword of a fundamental type, or a name that names
/// a type, with an optional qualifier, or a template-id.
pub fn simple_type(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(FUNDAMENTAL) {
        let k = p.bump();
        return Ok(p.node("simple_type", sp, vec![k]));
    }
    let mut kids = Vec::new();
    let mut q = Qual::None;
    let mut last = None;
    if p.at("::") || p.at("decltype") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Some((nn, qq, l)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
            q = qq;
            last = l;
        }
    }
    if p.kind() != TK::Ident {
        return Err(p.fail("a type name"));
    }
    let name = p.peek();
    // A constructor's declarator: the class's own name and `(`.
    if p.ctor_check && p.ahead(1) == "(" && (last == Some(name) || (q == Qual::None && p.names.class_name().as_deref() == Some(name))) {
        return Err(p.fail("a type name"));
    }
    if p.ahead(1) == "<" {
        let f = p.find(q, name);
        let is_t = f.is_template() || (q == Qual::Dependent && p.type_only);
        if is_t && matches!(f.kind, Kind::Template(TKind::Class | TKind::Alias) | Kind::Unknown) {
            let (t, _) = template_id(p, q, false)?;
            kids.push(Kid::Node(t));
            return Ok(p.node("simple_type", sp, kids));
        }
    }
    if !p.names_type(q, name) {
        return Err(p.fail("a type name"));
    }
    kids.push(identifier(p)?);
    Ok(p.node("simple_type", sp, kids))
}

/// Whether a type can start here, for the choice between a cast and a
/// parenthesized expression, and the operand of `sizeof`.
pub fn type_start(p: &mut Parser) -> bool {
    if p.at_any(FUNDAMENTAL) || p.at_any(&["const", "volatile", "class", "struct", "union", "enum", "typename", "auto", "decltype"]) {
        return true;
    }
    if !(p.kind() == TK::Ident || p.at("::")) {
        return false;
    }
    let m = p.mark();
    let mut q = Qual::None;
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Ok((_, qq, _)) = nested_name(p) {
            q = qq;
        }
    }
    let r = p.kind() == TK::Ident && {
        let name = p.peek();
        let f = p.find(q, name);
        f.is_type() || f.kind == Kind::Template(TKind::Concept) || (q == Qual::Dependent && p.type_only)
    };
    p.reset(m);
    r
}
