// tidal-fe: a hobby C++ front end built from the Tidal specification.
//
//   tidal-fe check FILE [--m32] [--werror] [-Wname]   exit 0 accept, 1 reject, 2 unsupported
//   tidal-fe run FILE [FILE...] [--m32]       run a program on the interpreter
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
mod interp;
mod ics;
mod lexer;
mod ovl;
mod parser;
mod special;
mod stmt;
mod templ;
mod types;

use diag::Severity;

pub enum Outcome {
    Accept,
    Reject(Vec<diag::Diag>),
    Unsupported(Vec<diag::Diag>),
}

pub fn parse_source(src: &str, path: Option<&std::path::Path>, m32: bool, werror: bool, wflags: &[String]) -> parser::P {
    types::ILP32.with(|c| c.set(m32));
    let mut d = diag::Diags::default();
    let toks = lexer::lex_file(src, path, &mut d);
    let mut p = parser::P::new(toks, d);
    p.werror = werror;
    p.wflags = wflags.to_vec();
    p.parse_translation_unit();
    p
}

pub fn outcome_of(p: &parser::P) -> (Outcome, Vec<diag::Diag>) {
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

pub fn check_source(src: &str, path: Option<&std::path::Path>, m32: bool, werror: bool, wflags: &[String]) -> (Outcome, Vec<diag::Diag>) {
    let p = parse_source(src, path, m32, werror, wflags);
    outcome_of(&p)
}

/// The exit statuses of `tidal-fe run` other than a program's own.
pub const RUN_UNDEFINED: i32 = 126;
pub const RUN_UNSUPPORTED: i32 = 127;
pub const RUN_REJECTED: i32 = 125;
/// A program that ends without returning from `main`, as `std::abort`
/// does on the matrix targets (128 + SIGABRT).
pub const RUN_TERMINATED: i32 = 134;

/// One line that describes a run's outcome.
pub fn describe_run(o: &interp::run::RunOutcome) -> String {
    use interp::run::RunOutcome;
    match o {
        RunOutcome::Result(v, ch) if ch.is_empty() => format!("result: {}", v),
        RunOutcome::Result(v, ch) => {
            let list: Vec<String> = ch.iter().map(|c| format!("{} at line {} ({})", c.rule, c.line, c.what)).collect();
            format!("unspecified: result {} with fixed choices: {}", v, list.join("; "))
        }
        RunOutcome::Undefined(r, m, l) => format!("undefined [{}] at line {}: {}", r, l, m),
        RunOutcome::Unsupported(m, l) => format!("unsupported at line {}: {}", l, m),
        RunOutcome::Terminated(r, m, l) => format!("terminated [{}] at line {}: {}", r, l, m),
    }
}

fn run_command(paths: &[String], m32: bool) -> ! {
    let mut srcs = Vec::new();
    for path in paths {
        match std::fs::read_to_string(path) {
            Ok(s) => srcs.push((path.clone(), s)),
            Err(e) => {
                eprintln!("{}: {}", path, e);
                std::process::exit(64);
            }
        }
    }
    let h = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || {
            // Each translation unit is checked on its own (TR-4), then the
            // units run linked together.
            let ps: Vec<parser::P> = srcs.iter().map(|(path, src)| parse_source(src, Some(std::path::Path::new(path)), m32, false, &[])).collect();
            for p in &ps {
                match outcome_of(p).0 {
                    Outcome::Reject(d) => {
                        let d = &d[0];
                        println!("rejected [{}] at line {}: {}", d.rule.unwrap_or("-"), d.line, d.msg);
                        return RUN_REJECTED;
                    }
                    Outcome::Unsupported(d) => {
                        println!("unsupported at line {}: check: {}", d[0].line, d[0].msg);
                        return RUN_UNSUPPORTED;
                    }
                    Outcome::Accept => {}
                }
            }
            let refs: Vec<&parser::P> = ps.iter().collect();
            let o = interp::run::run_units(&refs);
            println!("{}", describe_run(&o));
            match o {
                interp::run::RunOutcome::Result(v, _) => (v & 0xff) as i32,
                interp::run::RunOutcome::Undefined(..) => RUN_UNDEFINED,
                interp::run::RunOutcome::Unsupported(..) => RUN_UNSUPPORTED,
                interp::run::RunOutcome::Terminated(..) => RUN_TERMINATED,
            }
        })
        .unwrap();
    let code = h.join().unwrap_or(RUN_UNSUPPORTED);
    std::process::exit(code)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: tidal-fe check FILE [--m32] [--werror]\n       tidal-fe run FILE [FILE...] [--m32]\n       tidal-fe conform DIR [-v] [--out FILE]");
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
            let wflags: Vec<String> = args.iter().filter(|a| a.starts_with("-W")).cloned().collect();
            let (out, all) = check_source(&src, Some(std::path::Path::new(path)), m32, werror, &wflags);
            for d in &all {
                let kind = match d.sev {
                    Severity::Error => "error",
                    Severity::Unsupported => "unsupported",
                    Severity::Warning => "warning",
                };
                match d.rule {
                    Some(r) if d.cause.is_some() => eprintln!("{}:{}: {} [{}, {}]: {}", path, d.line, kind, r, d.cause.unwrap(), d.msg),
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
        "dump" => {
            // Development aid: the trees of one function (default main).
            let src = std::fs::read_to_string(&args[2]).unwrap_or_default();
            let want = args.get(3).cloned().unwrap_or("main".into());
            let p = parse_source(&src, Some(std::path::Path::new(&args[2])), false, false, &[]);
            for (i, d) in p.decls.iter().enumerate() {
                if want == "-" {
                    println!("{} {} {}", i, d.name, match &d.k { parser::DK::Func(_) => "func", parser::DK::Var(_) => "var", parser::DK::Field(_) => "field", parser::DK::Class(_) => "class", _ => "" });
                } else if d.name == want {
                    match &d.k {
                        parser::DK::Func(f) => {
                            println!("{} {} body:\n{}", i, d.name, f.body.as_ref().map(|b| ast::sstmt(b, 2)).unwrap_or_default());
                            for m in &f.mem_inits {
                                println!("  mem-init field {:?} base {:?}: {}", m.field, m.base, m.init.as_ref().map(|e| ast::sexp(e)).unwrap_or_default());
                            }
                        }
                        parser::DK::Var(v) => println!("{} {}: {}", i, d.name, v.init.as_ref().map(|e| ast::sexp(e)).unwrap_or_default()),
                        _ => {}
                    }
                }
            }
            std::process::exit(0)
        }
        "run" => {
            let files: Vec<String> = args[2..].iter().filter(|a| !a.starts_with("--")).cloned().collect();
            run_command(&files, args.iter().any(|a| a == "--m32"))
        }
        "conform" => conform::run(&args[2..]),
        _ => {
            eprintln!("unknown command {}", args[1]);
            std::process::exit(64);
        }
    }
}
