// tidal-fe: a hobby C++ front end built from the Tidal specification.
//
//   tidal-fe check FILE [--m32] [--werror]   exit 0 accept, 1 reject, 2 unsupported
//   tidal-fe conform DIR [-v] [--out FILE]    run the conformance tests

mod ast;
mod class;
mod conform;
mod consteval;
mod decl;
mod decl2;
mod diag;
mod expr;
mod expr2;
mod lexer;
mod parser;
mod stmt;
mod types;

use diag::Severity;

pub enum Outcome {
    Accept,
    Reject(Vec<diag::Diag>),
    Unsupported(Vec<diag::Diag>),
}

pub fn check_source(src: &str, m32: bool, werror: bool) -> (Outcome, Vec<diag::Diag>) {
    types::ILP32.with(|c| c.set(m32));
    let mut d = diag::Diags::default();
    let toks = lexer::lex(src, &mut d);
    let mut p = parser::P::new(toks, d);
    p.werror = werror;
    p.parse_translation_unit();
    let all = p.d.list.clone();
    let errors: Vec<_> = all.iter().filter(|d| d.sev == Severity::Error).cloned().collect();
    let unsup: Vec<_> = all.iter().filter(|d| d.sev == Severity::Unsupported).cloned().collect();
    let out = if !errors.is_empty() {
        Outcome::Reject(errors)
    } else if !unsup.is_empty() {
        Outcome::Unsupported(unsup)
    } else {
        Outcome::Accept
    };
    (out, all)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: tidal-fe check FILE [--m32] [--werror]\n       tidal-fe conform DIR [-v] [--out FILE]");
        std::process::exit(64);
    }
    match args[1].as_str() {
        "check" => {
            let path = &args[2];
            let m32 = args.iter().any(|a| a == "--m32");
            let werror = args.iter().any(|a| a == "--werror");
            let src = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    std::process::exit(64);
                }
            };
            let (out, all) = check_source(&src, m32, werror);
            for d in &all {
                let kind = match d.sev {
                    Severity::Error => "error",
                    Severity::Unsupported => "unsupported",
                    Severity::Warning => "warning",
                };
                match d.rule {
                    Some(r) => eprintln!("{}:{}: {} [{}]: {}", path, d.line, kind, r, d.msg),
                    None => eprintln!("{}:{}: {}: {}", path, d.line, kind, d.msg),
                }
            }
            match out {
                Outcome::Accept => std::process::exit(0),
                Outcome::Reject(_) => std::process::exit(1),
                Outcome::Unsupported(_) => std::process::exit(2),
            }
        }
        "conform" => conform::run(&args[2..]),
        _ => {
            eprintln!("unknown command {}", args[1]);
            std::process::exit(64);
        }
    }
}
