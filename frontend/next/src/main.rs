//! tidal-next: the rewrite of the clean-room front end (decision 0060,
//! `frontend/DESIGN-rewrite.md`). Build steps 1 and 2: the source map,
//! the diagnostics, the lexer, the parser with the `Names` oracle, and
//! the driver. The old front end (`frontend/src`, tidal-fe) stays the
//! front end of record until this one owns a test.
//!
//!   tidal-next check FILE [--m32] [--werror] [-Wname] [--proposal ID]
//!       exit 0 accept, 1 reject, 2 unsupported; with no analysis yet, a
//!       file that parses is unsupported
//!   tidal-next run FILE [FILE...] [--m32] [--proposal ID]
//!   tidal-next conform DIR [-v]
//!   tidal-next dump FILE        the syn trees, one declaration a line
//!   tidal-next lex FILE...      one token a line (kind, text, line, literal)
//!   tidal-next parse FILE...    one line a file: `tree` or `stop LINE: why`

#![allow(dead_code)] // the shapes and queries that later build steps call

mod classes;
mod declarations;
mod diag;
mod exceptions;
mod expressions;
mod foundations;
mod lex;
mod overloading;
mod proposals;
mod sem;
mod source;
mod statements;
mod syntax;
mod templates;

use diag::{Diags, Severity};
use std::path::Path;

/// The exit statuses of `tidal-fe run` other than a program's own.
pub const RUN_UNSUPPORTED: i32 = 127;
pub const RUN_REJECTED: i32 = 125;

/// What a unit gave: its tokens, the lexer's diagnostics, and the parse.
pub struct Parsed {
    pub lexed: lex::Lexed,
    pub diags: Diags,
    pub trees: Result<Vec<syntax::Node>, syntax::Stop>,
}

/// Lex and parse one translation unit with the step 2 oracle.
pub fn parse_file(src: &str, path: &Path, alts: &[syntax::Alt]) -> Parsed {
    let mut d = Diags::default();
    let lexed = lex::lex_file(src, Some(path), &mut d);
    let mut names = sem::names::Declare::default();
    let trees = syntax::parse_unit(&lexed.toks, &mut names, alts);
    Parsed { lexed, diags: d, trees }
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
    let alts = alternatives(&args);
    let Some(path) = files(&args).into_iter().next() else {
        eprintln!("usage: tidal-next check FILE");
        return 64;
    };
    let src = read(&path);
    let p = parse_file(&src, Path::new(&path), alts);
    let all = check_diags(&p);
    for d in &all {
        eprintln!("{}", d.render(&path, &p.lexed.map));
    }
    if all.iter().any(|d| d.sev == Severity::Error) {
        return 1;
    }
    let line = p.lexed.map.unit_line(p.lexed.toks[0].span);
    eprintln!("{}:{}: unsupported: no analysis before build step 3 (the unit parses)", path, line);
    2
}

fn run(args: Vec<String>) -> i32 {
    let alts = alternatives(&args);
    for path in files(&args) {
        let src = read(&path);
        let p = parse_file(&src, Path::new(&path), alts);
        if let Some(d) = check_diags(&p).iter().find(|d| d.sev == Severity::Error) {
            println!("rejected [{}] at line {}: {}", d.rule.unwrap_or("-"), p.lexed.map.unit_line(d.span), d.msg);
            return RUN_REJECTED;
        }
    }
    println!("unsupported at line 0: no machine before build step 3");
    RUN_UNSUPPORTED
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
    for path in files(&args) {
        let src = read(&path);
        let pth = path.clone();
        let r = big_stack(move || {
            let p = parse_file(&src, Path::new(&pth), alts);
            match &p.trees {
                Ok(_) => "tree".to_string(),
                Err(s) => {
                    let line = p.lexed.map.unit_line(p.lexed.toks[s.tok.min(p.lexed.toks.len() - 1)].span);
                    format!("stop {}: {}", line, syntax::stop_message(&p.lexed.toks, s))
                }
            }
        });
        println!("{}\t{}", r.unwrap_or_else(|| "crash".to_string()), path);
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
            eprintln!("conform: the harness moves with the machine at build step 3; `tidal-next parse` gives the parse of each file");
            64
        }
        _ => {
            eprintln!("unknown command {}", args[1]);
            64
        }
    };
    std::process::exit(code)
}
