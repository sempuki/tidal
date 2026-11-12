//! tidal-next: the rewrite of the clean-room front end (decision 0060,
//! `frontend/DESIGN-rewrite.md`). Build steps 1 and 2 give the source
//! map, the diagnostics, the lexer, and the parser with the `Names`
//! oracle; build step 3 gives analysis (`sem` and the `check.rs` parts),
//! the machine (`machine` and the `run.rs` parts), `target`, and the
//! harness (`conform`). The old front end (`frontend/src`, tidal-fe)
//! stays the front end of record for each test that the takeover list
//! does not name.
//!
//!   tidal-next check FILE [--m32] [--werror] [-Wname] [--proposal ID]
//!       exit 0 accept, 1 reject, 2 unsupported
//!   tidal-next run FILE [--m32] [--proposal ID]
//!   tidal-next conform DIR [-v] [--proposal ID]
//!   tidal-next dump FILE        the syn trees, one declaration a line
//!   tidal-next lex FILE...      one token a line (kind, text, line, literal)
//!   tidal-next parse FILE... [--all]
//!       one line a file: `tree` or `stop LINE: why`; with --all, the
//!       other stops follow the path, a tab before each

#![allow(dead_code)] // the shapes and queries that later build steps call

mod classes;
mod conform;
mod declarations;
mod diag;
mod exceptions;
mod expressions;
mod foundations;
mod lex;
mod machine;
mod overloading;
mod proposals;
mod sem;
mod source;
mod statements;
mod syntax;
mod target;
mod templates;

use diag::{Diags, Severity};
use std::path::Path;

/// The exit statuses of `tidal-fe run` other than a program's own.
pub const RUN_UNSUPPORTED: i32 = 127;
pub const RUN_REJECTED: i32 = 125;
pub const RUN_UNDEFINED: i32 = 126;

/// What a unit gave: its tokens, the lexer's diagnostics, and the parse.
pub struct Parsed {
    pub lexed: lex::Lexed,
    pub diags: Diags,
    pub trees: Result<Vec<syntax::Node>, syntax::Stop>,
    /// The names that the declare pass recorded.
    pub names: std::collections::HashSet<String>,
}

/// Lex and parse one translation unit with the step 2 oracle.
pub fn parse_file(src: &str, path: &Path, alts: &[syntax::Alt]) -> Parsed {
    let mut d = Diags::default();
    let lexed = lex::lex_file(src, Some(path), &mut d);
    let mut names = sem::names::Declare::default();
    let trees = syntax::parse_unit(&lexed.toks, &mut names, alts);
    Parsed { lexed, diags: d, trees, names: names.all_names() }
}

/// The diagnostics of a check: the lexer's, and the TR-12 stop.
fn check_diags(p: &Parsed) -> Vec<diag::Diag> {
    let mut all = p.diags.list.clone();
    if let Err(s) = &p.trees {
        let span = p.lexed.toks[s.tok.min(p.lexed.toks.len() - 1)].span;
        all.push(diag::Diag { sev: Severity::Error, rule: Some("TR-12"), cause: None, msg: syntax::stop_message(&p.lexed.toks, s), span });
    }
    all
}

/// The options of a check.
#[derive(Clone, Default)]
pub struct Options {
    pub m32: bool,
    pub werror: bool,
    pub wflags: Vec<String>,
    pub proposal: Option<&'static str>,
}

/// The answer for one translation unit.
pub enum Outcome {
    Accept,
    Reject(Vec<diag::Diag>),
    Unsupported(Vec<diag::Diag>),
}

/// Lex, parse, and analyze one unit: the outcome, and the unit when the
/// analysis ran.
pub fn check_source(src: &str, path: &Path, o: &Options) -> (Outcome, Option<sem::Unit>) {
    let alts = o.proposal.and_then(proposals::find).map(|p| p.alternatives).unwrap_or(&[]);
    let p = parse_file(src, path, alts);
    let mut all = check_diags(&p);
    let mut unit = None;
    if let Ok(trees) = &p.trees {
        let u = sem::Unit { target: target::Target { ilp32: o.m32 }, ..Default::default() };
        let mut cx = sem::check::Check::new(&p.lexed.toks, u);
        cx.werror = o.werror;
        cx.wflags = o.wflags.clone();
        cx.parser_names = p.names.clone();
        if let Some(pr) = o.proposal.and_then(proposals::find) {
            cx.hooks = (pr.hooks)();
        }
        declarations::check::translation_unit(&mut cx, trees);
        let mut u = cx.u;
        // A diagnostic's span becomes the line of the unit's main file.
        for d in u.diags.iter_mut() {
            d.span.line = p.lexed.map.unit_line(d.span);
            d.span.file = source::FileId(0);
        }
        all.extend(u.diags.iter().cloned());
        unit = Some(u);
    }
    for d in all.iter_mut() {
        d.span.line = p.lexed.map.unit_line(d.span);
        d.span.file = source::FileId(0);
    }
    let errors: Vec<_> = all.iter().filter(|d| d.sev == Severity::Error).cloned().collect();
    let unsup: Vec<_> = all.iter().filter(|d| d.sev == Severity::Unsupported).cloned().collect();
    let out = if !errors.is_empty() {
        Outcome::Reject(errors)
    } else if !unsup.is_empty() {
        Outcome::Unsupported(unsup)
    } else {
        Outcome::Accept
    };
    if let Some(u) = unit.as_mut() {
        u.diags = all;
    }
    (out, unit)
}

/// The prototype that `--proposal` names, if any.
pub fn proposal_of(args: &[String]) -> Option<&'static str> {
    let i = args.iter().position(|a| a == "--proposal")?;
    proposals::find(args.get(i + 1)?).map(|p| p.id)
}

fn options(args: &[String]) -> Options {
    Options {
        m32: args.iter().any(|a| a == "--m32"),
        werror: args.iter().any(|a| a == "--werror" || a == "-Werror"),
        wflags: args.iter().filter(|a| a.starts_with("-W") && *a != "-Werror").cloned().collect(),
        proposal: proposal_of(args),
    }
}

fn read(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", path, e);
            std::process::exit(64);
        }
    }
}

/// The alternatives of the prototype that `--proposal` names.
fn alternatives(args: &[String]) -> &'static [syntax::Alt] {
    let Some(i) = args.iter().position(|a| a == "--proposal") else { return &[] };
    let Some(id) = args.get(i + 1) else {
        eprintln!("--proposal needs an ID");
        std::process::exit(64);
    };
    match proposals::find(id) {
        Some(p) => p.alternatives,
        None => {
            eprintln!("no prototype is registered as {}", id);
            std::process::exit(64);
        }
    }
}

/// Run `f` on a thread with a large stack: the parser recurses on the
/// program's nesting.
fn big_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).ok()?.join().ok()
}

fn files(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if a == "--proposal" {
            skip = true;
            continue;
        }
        if !a.starts_with('-') {
            out.push(a.clone());
        }
    }
    out
}

fn check(args: Vec<String>) -> i32 {
    alternatives(&args);
    let Some(path) = files(&args).into_iter().next() else {
        eprintln!("usage: tidal-next check FILE");
        return 64;
    };
    let src = read(&path);
    let (out, unit) = check_source(&src, Path::new(&path), &options(&args));
    if let Some(u) = &unit {
        for d in &u.diags {
            eprintln!("{}", render(d, &path));
        }
    } else if let Outcome::Reject(ds) | Outcome::Unsupported(ds) = &out {
        for d in ds {
            eprintln!("{}", render(d, &path));
        }
    }
    match out {
        Outcome::Accept => 0,
        Outcome::Reject(_) => 1,
        Outcome::Unsupported(_) => 2,
    }
}

/// `FILE:LINE: kind [RULE]: msg`, the printed form of the old front end.
fn render(d: &diag::Diag, path: &str) -> String {
    let map = source::SourceMap::new(None);
    d.render(path, &map)
}

fn run(args: Vec<String>) -> i32 {
    alternatives(&args);
    let fs = files(&args);
    if fs.len() != 1 {
        println!("unsupported at line 0: a program of several translation units (linking comes with a later step)");
        return RUN_UNSUPPORTED;
    }
    let src = read(&fs[0]);
    let (out, unit) = check_source(&src, Path::new(&fs[0]), &options(&args));
    match out {
        Outcome::Reject(ds) => {
            let d = &ds[0];
            println!("rejected [{}] at line {}: {}", d.rule.unwrap_or("-"), d.span.line, d.msg);
            RUN_REJECTED
        }
        Outcome::Unsupported(ds) => {
            println!("unsupported at line {}: {}", ds[0].span.line, ds[0].msg);
            RUN_UNSUPPORTED
        }
        Outcome::Accept => {
            let o = foundations::translation::run::run_program(unit.as_ref().unwrap());
            println!("{}", foundations::translation::run::describe(&o));
            match o {
                foundations::translation::run::RunOutcome::Result(v) => (v & 0xff) as i32,
                foundations::translation::run::RunOutcome::Undefined(..) => RUN_UNDEFINED,
                foundations::translation::run::RunOutcome::Unsupported(..) => RUN_UNSUPPORTED,
            }
        }
    }
}

fn lex_cmd(args: Vec<String>) -> i32 {
    let fs = files(&args);
    for path in &fs {
        let src = read(path);
        let mut d = Diags::default();
        let lexed = lex::lex_file(&src, Some(Path::new(path)), &mut d);
        if fs.len() > 1 {
            println!("== {}", path);
        }
        print!("{}", lex::dump(&lexed, &d));
    }
    0
}

fn parse_cmd(args: Vec<String>) -> i32 {
    let alts = alternatives(&args);
    let all = args.iter().any(|a| a == "--all");
    for path in files(&args) {
        let src = read(&path);
        let pth = path.clone();
        let r = big_stack(move || {
            let p = parse_file(&src, Path::new(&pth), alts);
            let line = |tok: usize| p.lexed.map.unit_line(p.lexed.toks[tok.min(p.lexed.toks.len() - 1)].span);
            match &p.trees {
                Ok(_) => "tree".to_string(),
                Err(s) => {
                    let mut out = format!("stop {}: {}", line(s.tok), syntax::stop_message(&p.lexed.toks, s));
                    if all {
                        // The other places the grammar does not derive,
                        // from a parse that skips each failed form.
                        let mut names = sem::names::Declare::default();
                        let first = line(s.tok);
                        let mut seen = vec![first];
                        for o in syntax::parse_unit_all(&p.lexed.toks, &mut names, alts) {
                            let l = line(o.tok);
                            if !seen.contains(&l) {
                                seen.push(l);
                                out.push_str(&format!("\t{}: {}", l, syntax::stop_message(&p.lexed.toks, &o)));
                            }
                        }
                    }
                    out
                }
            }
        });
        let r = r.unwrap_or_else(|| "crash".to_string());
        match r.split_once('\t') {
            Some((first, more)) => println!("{}\t{}\t{}", first, path, more),
            None => println!("{}\t{}", r, path),
        }
    }
    0
}

fn dump(args: Vec<String>) -> i32 {
    let alts = alternatives(&args);
    let Some(path) = files(&args).into_iter().next() else { return 64 };
    let src = read(&path);
    let p = parse_file(&src, Path::new(&path), alts);
    match &p.trees {
        Ok(ts) => {
            for t in ts {
                println!("{}", t.sexp(&p.lexed.toks));
            }
            0
        }
        Err(s) => {
            println!("stop: {}", syntax::stop_message(&p.lexed.toks, s));
            1
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: tidal-next (check|run|conform|dump|lex|parse) ...");
        std::process::exit(64);
    }
    let rest: Vec<String> = args[2..].to_vec();
    let code = match args[1].as_str() {
        "check" => big_stack(move || check(rest)).unwrap_or(2),
        "run" => big_stack(move || run(rest)).unwrap_or(RUN_UNSUPPORTED),
        "dump" => big_stack(move || dump(rest)).unwrap_or(2),
        "lex" => lex_cmd(rest),
        "parse" => parse_cmd(rest),
        "conform" => {
            conform::run(&rest);
            0
        }
        _ => {
            eprintln!("unknown command {}", args[1]);
            64
        }
    };
    std::process::exit(code)
}
