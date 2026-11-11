//! The Syntax segment of `classes/members`.

use crate::classes::access::syntax::{access_label, friend_decl};
use crate::classes::derived::syntax::class_head;
use crate::classes::r#virtual::syntax::{pure_specifier, virt_specifiers};
use crate::classes::unions::syntax::bitfield;
use crate::declarations::alignment::syntax::alignment_specifier;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::declares_function;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::declarations::namespaces::syntax::using_decl;
use crate::declarations::static_assert::syntax::static_assert_decl;
use crate::declarations::type_names::syntax::alias_decl;
use crate::declarations::{body_now, declarator_here, function_definition_rest, looks_like_guide, specs_here};
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::primary::syntax::identifier;
use crate::lex::TK;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::class_template_deduction::syntax::deduction_guide;
use crate::templates::concepts::syntax::requires_clause;
use crate::templates::declarations::syntax::template_decl;
use crate::templates::specialization::syntax::explicit_specialization;

/// `class_def: class_head "{" {member_decl} "}"`.
pub fn class_def(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let head = class_head(p)?;
    if !p.at("{") {
        return Err(p.fail("'{'"));
    }
    let tmpl = p.take_template();
    let scopes = p.declared(&head, DeclCtx { specs: None, template: tmpl, param: false });
    let mut kids = vec![Kid::Node(head)];
    kids.extend(class_body(p, scopes.last().copied())?);
    Ok(p.node("class_def", sp, kids))
}

/// `"{" {member_decl} "}"` in the class's scope. When the outermost class
/// closes, the bodies kept since it opened are parsed (NAM-6).
pub fn class_body(p: &mut Parser, scope: Option<u32>) -> PResult<Vec<Kid>> {
    let mut kids = vec![p.expect("{")?];
    let entered = scope.is_some();
    if let Some(s) = scope {
        p.names.enter(Enter::Scope(s));
    }
    p.class_depth += 1;
    let from = p.deferred.len();
    let r = p.bracketed(|p| -> PResult<()> {
        while !p.at("}") {
            if p.at_eof() {
                return Err(p.fail("'}'"));
            }
            kids.push(Kid::Node(member_decl(p)?));
        }
        kids.push(p.bump());
        Ok(())
    });
    p.class_depth -= 1;
    let r = r.and_then(|_| if p.class_depth == 0 { p.parse_deferred(from, body_now) } else { Ok(()) });
    if entered {
        p.names.leave();
    }
    r?;
    Ok(kids)
}

/// `class_name: identifier`.
pub fn class_name(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let id = identifier(p)?;
    Ok(p.node("class_name", sp, vec![id]))
}

/// `member_decl`: a member declaration, a function definition, an access
/// label, a friend, an alias, a using-declaration, a `static_assert`, a
/// template, an explicit specialization, or a deduction guide.
pub fn member_decl(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("member_decl") {
        return Ok(n);
    }
    let sp = p.span();
    let wrap = |p: &mut Parser, n: Node| Ok(p.node("member_decl", sp, vec![Kid::Node(n)]));
    if p.at_any(&["public", "protected", "private"]) {
        let n = access_label(p)?;
        return wrap(p, n);
    }
    if p.at("friend") {
        if let Ok(n) = p.attempt(friend_decl) {
            return wrap(p, n);
        }
    }
    if p.at("using") {
        let n = if p.ahead_kind(1) == TK::Ident && matches!(p.ahead(2), "=" | "[") { alias_decl(p)? } else { using_decl(p)? };
        return wrap(p, n);
    }
    if p.at("static_assert") {
        let n = static_assert_decl(p)?;
        return wrap(p, n);
    }
    if p.at("template") {
        let n = if p.ahead(2) == ">" { explicit_specialization(p)? } else { template_decl(p)? };
        return wrap(p, n);
    }
    if looks_like_guide(p) {
        let n = deduction_guide(p)?;
        return wrap(p, n);
    }
    let mut pre = Vec::new();
    while p.at_attr() || p.at("alignas") {
        pre.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
    }
    let specs = p.opt(specs_here);
    let mut kids = pre.clone();
    if let Some(s) = &specs {
        kids.push(Kid::Node(s.clone()));
    }
    if p.at(";") {
        kids.push(p.bump());
        return Ok(p.node("member_decl", sp, kids));
    }
    let tmpl = p.take_template();
    let first = if is_bitfield(p) {
        bitfield(p)?
    } else {
        let m = p.mark();
        let d = declarator_here(p)?;
        p.template = tmpl;
        if declares_function(&d) {
            if let Some(f) = p.opt(|p| function_definition_rest(p, sp, pre.clone(), specs.clone(), d.clone())) {
                return wrap(p, f);
            }
        }
        p.reset(m);
        let _ = p.take_template();
        let d = declarator_here(p)?;
        member_declarator_after(p, d, specs.as_ref(), tmpl)?
    };
    kids.push(Kid::Node(first));
    while p.at(",") {
        kids.push(p.bump());
        let n = if is_bitfield(p) {
            bitfield(p)?
        } else {
            let d = declarator_here(p)?;
            member_declarator_after(p, d, specs.as_ref(), false)?
        };
        kids.push(Kid::Node(n));
    }
    kids.push(p.expect(";")?);
    Ok(p.node("member_decl", sp, kids))
}

fn is_bitfield(p: &Parser) -> bool {
    p.at(":") || (p.kind() == TK::Ident && p.ahead(1) == ":")
}

/// `member_declarator: declarator [virt_specifiers] [pure_specifier | member_init]
/// | declarator requires_clause | bitfield`.
pub fn member_declarator(p: &mut Parser) -> PResult<Node> {
    if is_bitfield(p) {
        return bitfield(p);
    }
    let d = declarator_here(p)?;
    member_declarator_after(p, d, None, false)
}

fn member_declarator_after(p: &mut Parser, d: Node, specs: Option<&Node>, template: bool) -> PResult<Node> {
    let sp = d.span;
    p.declared(&d, DeclCtx { specs, template, param: false });
    let is_fn = declares_function(&d);
    let mut kids = vec![Kid::Node(d)];
    if p.at("requires") {
        kids.push(Kid::Node(requires_clause(p)?));
        return Ok(p.node("member_declarator", sp, kids));
    }
    if let Some(v) = p.opt(virt_specifiers) {
        kids.push(Kid::Node(v));
    }
    if is_fn && p.at("=") && p.ahead_kind(1) == TK::Int {
        kids.push(Kid::Node(pure_specifier(p)?));
    } else if p.at("=") || p.at("{") {
        kids.push(Kid::Node(member_init(p)?));
    }
    Ok(p.node("member_declarator", sp, kids))
}

/// `member_init: "=" assignment_expr | "=" braced_init | braced_init`.
pub fn member_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("{") {
        kids.push(Kid::Node(braced_init(p)?));
    } else {
        kids.push(p.expect("=")?);
        kids.push(Kid::Node(if p.at("{") { braced_init(p)? } else { assignment_expr(p)? }));
    }
    Ok(p.node("member_init", sp, kids))
}
