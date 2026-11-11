//! Unit tests of the syntax core: the cursor, the extension point, the
//! deferred bodies, and the check that each nonterminal of the Syntax
//! segments has one function, named after it, in its page's module.

use super::testing::*;
use super::{Alt, Kid, Node, PResult, Parser};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[test]
fn close_angle_splits_shift() {
    let t = toks(">> x");
    let mut st = stub();
    let mut p = Parser::new(&t, &mut st, &[]);
    assert!(p.close_angle().is_ok());
    assert_eq!(p.peek(), ">");
    assert!(p.close_angle().is_ok());
    assert_eq!(p.peek(), "x");
}

#[test]
fn attempt_resets_the_cursor() {
    let t = toks("a b c");
    let mut st = stub();
    let mut p = Parser::new(&t, &mut st, &[]);
    let r: PResult<()> = p.attempt(|p| {
        p.bump();
        p.bump();
        Err(p.fail("x"))
    });
    assert!(r.is_err());
    assert_eq!(p.peek(), "a");
    assert_eq!(p.furthest().tok, 2);
}

/// P22's syntax part, as the unit test of the nonterminal point:
/// `"break" identifier ";"` and `"continue" identifier ";"` at `jump_stmt`.
fn labeled_jump(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !(p.at("break") || p.at("continue")) {
        return Err(p.fail("'break' or 'continue'"));
    }
    let k = p.bump();
    let id = crate::expressions::primary::syntax::identifier(p)?;
    let semi = p.expect(";")?;
    Ok(p.node("labeled_jump", sp, vec![k, id, semi]))
}

#[test]
fn nonterminal_point_takes_a_proposal_alternative() {
    let alts = [Alt { nonterminal: "jump_stmt", proposal: "P22", parse: labeled_jump }];
    let t = toks("break outer;");
    let mut st = stub();
    let mut p = Parser::new(&t, &mut st, &alts);
    let n = crate::statements::jumps::syntax::jump_stmt(&mut p).unwrap();
    assert_eq!(n.ext, Some("P22"));
    assert_eq!(n.sexp(&t), "(labeled_jump ext:P22 break outer ;)");
    // Without the prototype the form is a TR-12 stop (JMP-17).
    assert!(!ok("break outer;", crate::statements::jumps::syntax::jump_stmt));
    // With it, the ordinary form is unchanged.
    let t2 = toks("break;");
    let mut p2 = Parser::new(&t2, &mut st, &alts);
    assert_eq!(crate::statements::jumps::syntax::jump_stmt(&mut p2).unwrap().sexp(&t2), "(jump_stmt break ;)");
}

#[test]
fn member_bodies_wait_for_the_class() {
    // The body uses a type declared after it (NAM-6).
    let src = "struct S { int f() { Inner i; return 0; } struct Inner {}; };";
    let u = unit(src).unwrap();
    assert!(u[0].contains("(function_body (compound_stmt {"));
    assert!(!u[0].contains("<deferred"));
}

#[test]
fn a_stop_names_the_furthest_token() {
    let e = unit("int f() { return 1 + ; }").unwrap_err();
    assert!(e.contains("found ';'"), "{}", e);
}

fn spec_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../specification")
}

/// The nonterminals each page defines, from its Syntax segments.
fn nonterminals() -> BTreeMap<String, Vec<String>> {
    fn walk(d: &Path, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "md") {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(&spec_dir(), &mut files);
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in files {
        let text = std::fs::read_to_string(&f).unwrap();
        let page = f.strip_prefix(spec_dir()).unwrap().with_extension("").to_string_lossy().replace('\\', "/");
        let mut inside = false;
        for line in text.lines() {
            if line.contains(".. productionlist:: tidal") {
                inside = true;
                continue;
            }
            if inside && line.starts_with("```") {
                inside = false;
                continue;
            }
            if inside {
                let t = line.trim_start();
                if let Some(k) = t.find(':') {
                    let name = &t[..k];
                    if !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
                        out.entry(page.clone()).or_default().push(name.to_string());
                    }
                }
            }
        }
    }
    out
}

/// The source files of a page's syntax part.
fn syntax_files(page: &str) -> Vec<PathBuf> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let (dir, last) = page.rsplit_once('/').unwrap_or(("", page));
    if last == "index" {
        return vec![src.join(dir).join("syntax.rs")];
    }
    let module = match last {
        "if" => "if_stmt".to_string(),
        other => other.replace('-', "_"),
    };
    let d = src.join(dir).join(module);
    ["syntax.rs", "lexical.rs"].iter().map(|f| d.join(f)).filter(|p| p.exists()).collect()
}

/// The functions a source text defines, with the production its doc
/// comment names first, if any.
fn functions(text: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let mut doc = String::new();
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(d) = t.strip_prefix("///") {
            if doc.is_empty() {
                doc = d.trim().to_string();
            }
            continue;
        }
        if let Some(rest) = t.strip_prefix("pub fn ").or_else(|| t.strip_prefix("fn ")) {
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            let named = doc.strip_prefix('`').map(|d| d.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>());
            let produces = named.filter(|n| doc.starts_with(&format!("`{}:", n)) || doc.starts_with(&format!("`{}`", n)));
            out.push((name, produces));
        }
        doc.clear();
    }
    out
}

#[test]
fn each_nonterminal_has_one_function_in_its_page() {
    let nts = nonterminals();
    let total: usize = nts.values().map(Vec::len).sum();
    assert_eq!(total, 260, "the Syntax segments hold 260 nonterminals");
    assert_eq!(nts.len(), 62, "on 62 pages");
    let mut missing = Vec::new();
    let mut doubled = Vec::new();
    let mut all_fns: BTreeMap<String, usize> = BTreeMap::new();
    let mut wrong_doc = Vec::new();
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![src.clone()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if matches!(p.file_name().and_then(|n| n.to_str()), Some("syntax.rs" | "lexical.rs")) && !p.starts_with(src.join("syntax")) {
                for (name, produces) in functions(&std::fs::read_to_string(&p).unwrap()) {
                    *all_fns.entry(name.clone()).or_default() += 1;
                    if let Some(prod) = produces {
                        if prod != name {
                            wrong_doc.push(format!("{} documents `{}`", name, prod));
                        }
                    }
                }
            }
        }
    }
    for (page, names) in &nts {
        let files = syntax_files(page);
        assert!(!files.is_empty(), "no syntax part for {}", page);
        let text: String = files.iter().map(|f| std::fs::read_to_string(f).unwrap()).collect();
        let fns: Vec<String> = functions(&text).into_iter().map(|(n, _)| n).collect();
        for n in names {
            match fns.iter().filter(|f| *f == n).count() {
                0 => missing.push(format!("{}: {}", page, n)),
                1 => {}
                _ => doubled.push(format!("{}: {}", page, n)),
            }
            if all_fns.get(n).copied().unwrap_or(0) > 1 {
                doubled.push(format!("{} has a function in more than one page", n));
            }
        }
    }
    assert!(missing.is_empty(), "nonterminals with no function: {:?}", missing);
    assert!(doubled.is_empty(), "nonterminals with two functions: {:?}", doubled);
    assert!(wrong_doc.is_empty(), "functions named after another production: {:?}", wrong_doc);
}

#[test]
fn trees_name_their_productions() {
    let s = sx("x + 1", crate::expressions::comma::syntax::expression).unwrap();
    assert_eq!(s, "(add_expr (primary (id_expression (unqualified_id x))) + (primary (literal 1)))");
    let _ = Kid::Tok(0);
}
