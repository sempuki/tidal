//! The Syntax segment of `classes/copy-move`: shapes of constructors and
//! assignment operators, as recognizers over declaration trees.

use crate::classes::constructors::syntax::parts;
use crate::lex::Token;
use crate::syntax::Node;

fn words(n: &Node, toks: &[Token]) -> Vec<String> {
    let mut ids = Vec::new();
    n.tokens(&mut ids);
    ids.iter().map(|&i| toks[i].text.clone()).collect()
}

/// The parameters of a declaration's function declarator.
fn params(n: &Node) -> Vec<&Node> {
    let (_, d) = parts(n);
    d.and_then(|d| d.find("parameters")).map(|ps| ps.children("parameter").collect()).unwrap_or_default()
}

/// The parameter's type words, without its name and default.
fn param_type(prm: &Node, toks: &[Token]) -> Vec<String> {
    let mut w = words(prm.child("decl_specifiers").unwrap_or(prm), toks);
    if let Some(d) = prm.child("declarator").or(prm.child("abstract_declarator")) {
        for x in words(d, toks) {
            if x == "&" || x == "&&" || x == "*" {
                w.push(x);
            }
        }
    }
    w
}

/// `copy_param: class_name | [cv] class_name "&"`.
pub fn copy_param(w: &[String], class: &str) -> bool {
    let w: Vec<&str> = w.iter().map(String::as_str).filter(|x| *x != "const" && *x != "volatile").collect();
    w == [class] || w == [class, "&"]
}

/// `default_param: "," parameter`, a parameter with a default argument.
pub fn default_param(prm: &Node, toks: &[Token]) -> bool {
    prm.words(toks).contains(&"=")
}

fn ctor_with(n: &Node, toks: &[Token], class: &str, refs: &str) -> bool {
    let (_, d) = parts(n);
    let Some(d) = d else { return false };
    let w = words(d, toks);
    if w.first().map(String::as_str) != Some(class) || w.get(1).map(String::as_str) != Some("(") {
        return false;
    }
    let ps = params(n);
    let Some(first) = ps.first() else { return false };
    let t = param_type(first, toks);
    let core: Vec<&str> = t.iter().map(String::as_str).filter(|x| *x != "const" && *x != "volatile").collect();
    core == [class, refs] && ps[1..].iter().all(|q| default_param(q, toks))
}

/// `copy_ctor: class_name "(" [cv] class_name "&" {default_param} ")"`.
pub fn copy_ctor(n: &Node, toks: &[Token], class: &str) -> bool {
    ctor_with(n, toks, class, "&")
}

/// `move_ctor: class_name "(" [cv] class_name "&&" {default_param} ")"`.
pub fn move_ctor(n: &Node, toks: &[Token], class: &str) -> bool {
    ctor_with(n, toks, class, "&&")
}

fn assign_with(n: &Node, toks: &[Token], class: &str, ok: impl Fn(&[String]) -> bool) -> bool {
    let (_, d) = parts(n);
    let Some(d) = d else { return false };
    let Some(op) = d.find("overloadable_operator") else { return false };
    if words(op, toks) != ["="] {
        return false;
    }
    let ps = params(n);
    ps.len() == 1 && ok(&param_type(ps[0], toks)) && !class.is_empty()
}

/// `copy_assign: type_id "operator" "=" "(" copy_param ")"`.
pub fn copy_assign(n: &Node, toks: &[Token], class: &str) -> bool {
    assign_with(n, toks, class, |t| copy_param(t, class))
}

/// `move_assign: type_id "operator" "=" "(" [cv] class_name "&&" ")"`.
pub fn move_assign(n: &Node, toks: &[Token], class: &str) -> bool {
    assign_with(n, toks, class, |t| {
        let core: Vec<&str> = t.iter().map(String::as_str).filter(|x| *x != "const" && *x != "volatile").collect();
        core == [class, "&&"]
    })
}
