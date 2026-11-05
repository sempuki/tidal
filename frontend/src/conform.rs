// Conformance harness: runs every test under a directory and compares the
// front end's answer with the test's expectation.

use crate::diag::Diag;
use crate::interp::run::{run_explored, RunOutcome};
use crate::{outcome_of, parse_source, Outcome};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

struct Test {
    path: PathBuf,
    rel: String,
    /// The base expectation (`tidal-expect`) is `reject`. The harness
    /// judges by this expectation.
    expect_reject: bool,
    /// The test has no base expectation (`tidal-expect`), only lines for
    /// single compilers. The harness cannot judge its answer and gives it
    /// the status `NO-BASE` (G135).
    no_base: bool,
    /// The EDG override, when there is one and it differs from the base
    /// expectation on reject against accept.
    edg_reject: Option<bool>,
    /// The rules allow EDG's answer as well as the base one. With a
    /// `tidal-allows-edg` line, the rules that line names decide; without
    /// one, every rule the test cites must be `impl-defined`,
    /// `unspecified`, `no-diagnostic`, or `conditionally-supported`.
    edg_allowed: bool,
    rules: Vec<String>,
    m32: bool,
    /// The limit on nested instantiations that the flags set (TR-13).
    depth: Option<u32>,
    werror: bool,
    /// The -W flags other than -Werror.
    wflags: Vec<String>,
    sources: Vec<PathBuf>,
    /// The base expectation is `run` or `run-fail`, or the test has a
    /// `tidal-interpreter: run-fail` line: the harness runs the program
    /// after the check accepts it.
    run: Option<RunExpect>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum RunExpect {
    Run,
    RunFail,
    /// `tidal-interpreter: run-fail` on an `accept` test (decision 0053):
    /// the run must stop at undefined behavior and name one of the test's
    /// rules.
    Undefined,
}

/// The result of running a `run` or `run-fail` test.
#[derive(Clone, PartialEq, Debug)]
enum RunRes {
    Pass,
    /// Every explored run gives an allowed outcome, but the exploration
    /// is partial: the bound stopped it, or another run is unsupported.
    PassPartial,
    /// The first run gives an allowed outcome, and another explored run
    /// gives an outcome that the expectation does not allow.
    OtherOutcome,
    /// A `run` test ended with a nonzero status, or a `run-fail` test whose
    /// rules name no undefined behavior ended with status 0.
    WrongResult,
    /// A `run-fail` test ended with status 0.
    MissedUndefined,
    /// The run stopped at undefined behavior that the test does not expect.
    UnexpectedUndefined,
    Unsupported,
    /// The check did not accept the program, so it did not run.
    NotRun,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Res {
    Pass,
    FalseReject,
    MissedReject,
    Unsupported,
    /// Agrees with the base expectation where the EDG override differs.
    EdgOnly,
    /// Agrees with the EDG override against the base expectation. A
    /// failure.
    EdgAgrees,
    /// Agrees with the EDG override against the base expectation, and
    /// the rules allow either answer. A pass.
    EdgAllowed,
    /// The test has no base expectation, so its answer cannot be judged.
    /// A failure.
    NoBase,
}

/// The rules of the specification: the kind of each rule (`:kind:` under
/// `:::{rule} ID`), and the rules that its text links to (each
/// {rule}`ID` between the rule's fences).
struct Rules {
    kinds: std::collections::HashMap<String, String>,
    links: std::collections::HashMap<String, Vec<String>>,
}

fn read_rules(spec: &Path) -> Rules {
    let mut out = std::collections::HashMap::new();
    let mut links: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut files = Vec::new();
    fn walk(d: &Path, out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|e| e == "md") {
                out.push(p);
            }
        }
    }
    walk(spec, &mut files);
    for f in files {
        let Ok(src) = std::fs::read_to_string(&f) else { continue };
        // The rule whose kind line comes next, and the rule whose text
        // the current line is in.
        let mut id: Option<String> = None;
        let mut body: Option<String> = None;
        for line in src.lines() {
            if let Some(r) = line.strip_prefix(":::{rule}") {
                id = Some(r.trim().to_string());
                body = id.clone();
                continue;
            }
            if let (Some(i), Some(k)) = (&id, line.strip_prefix(":kind:")) {
                out.insert(i.clone(), k.trim().to_string());
            }
            id = None;
            if line.trim() == ":::" {
                body = None;
            } else if let Some(b) = &body {
                let mut rest = line;
                while let Some(i) = rest.find("{rule}`") {
                    rest = &rest[i + 7..];
                    if let Some(j) = rest.find('`') {
                        let target = rest[..j].to_string();
                        let l = links.entry(b.clone()).or_default();
                        if target != *b && !l.contains(&target) {
                            l.push(target);
                        }
                        rest = &rest[j + 1..];
                    }
                }
            }
        }
    }
    Rules { kinds: out, links }
}

/// The rule kinds under which an implementation's answer can differ from
/// the base expectation and still be allowed.
const LENIENT: [&str; 4] = ["impl-defined", "unspecified", "no-diagnostic", "conditionally-supported"];

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().is_some_and(|e| e == "cpp") && !p.to_string_lossy().ends_with(".tu.cpp") {
            out.push(p);
        }
    }
}

fn parse_test(root: &Path, path: &Path, kinds: &std::collections::HashMap<String, String>) -> Option<Test> {
    let src = std::fs::read_to_string(path).ok()?;
    let mut expect = None;
    let mut edg = None;
    // The expectations for single compilers, for a test with no base
    // expectation.
    let mut each = Vec::new();
    let mut rules = Vec::new();
    let mut flags = String::new();
    let mut sources = Vec::new();
    let mut interpreter = None;
    // The rules that allow EDG's answer (G134).
    let mut allows_edg = Vec::new();
    for line in src.lines() {
        let Some(rest) = line.strip_prefix("// tidal-") else {
            if !line.starts_with("//") {
                break;
            }
            continue;
        };
        if let Some(v) = rest.strip_prefix("expect:") {
            expect = Some(v.trim().to_string());
        } else if let Some(v) = rest.strip_prefix("allows-edg:") {
            allows_edg.extend(v.split_whitespace().map(String::from));
        } else if let Some(v) = rest.strip_prefix("expect-edg:") {
            edg = Some(v.trim().to_string());
            each.push(v.trim() == "reject");
        } else if let Some((_, v)) = rest.strip_prefix("expect-").and_then(|r| r.split_once(':')) {
            each.push(v.trim() == "reject");
        } else if let Some(v) = rest.strip_prefix("rules:") {
            rules = v.split_whitespace().map(String::from).collect();
        } else if let Some(v) = rest.strip_prefix("interpreter:") {
            interpreter = Some(v.trim().to_string());
        } else if let Some(v) = rest.strip_prefix("flags:") {
            flags = v.trim().to_string();
        } else if let Some(v) = rest.strip_prefix("sources:") {
            for s in v.split_whitespace() {
                sources.push(path.parent().unwrap().join(s));
            }
        }
    }
    // A front end only checks programs: run, run-fail, and link-fail mean
    // accept. The base expectation decides; the EDG override only names
    // a second answer.
    // A test with no base expectation gives one per compiler. The harness
    // does not choose among them (G135).
    let no_base = expect.is_none();
    let expect = match expect {
        Some(e) => e,
        None if each.is_empty() => return None,
        None => if each.iter().all(|&r| r) { "reject".to_string() } else { "accept".to_string() },
    };
    let expect_reject = expect == "reject";
    let edg_reject = if no_base { None } else { edg.map(|x| x == "reject").filter(|&r| r != expect_reject) };
    let lenient = |r: &String| kinds.get(r).is_some_and(|k| LENIENT.contains(&k.as_str()));
    // A `tidal-allows-edg` line names the rules that allow EDG's answer
    // (G134). Without one, every rule the test cites must allow it.
    let edg_allowed = if allows_edg.is_empty() {
        !rules.is_empty() && rules.iter().all(lenient)
    } else {
        allows_edg.iter().all(lenient)
    };
    let run = match Some(expect.as_str()) {
        Some("run") => Some(RunExpect::Run),
        Some("run-fail") => Some(RunExpect::RunFail),
        _ if interpreter.as_deref() == Some("run-fail") => Some(RunExpect::Undefined),
        _ => None,
    };
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
    Some(Test {
        path: path.to_path_buf(),
        rel,
        expect_reject,
        no_base,
        edg_reject,
        edg_allowed,
        rules,
        m32: flags.contains("-m32"),
        depth: crate::types::depth_limit_of(&flags.split_whitespace().collect::<Vec<_>>()),
        werror: flags.contains("-Werror"),
        wflags: flags.split_whitespace().filter(|f| f.starts_with("-W") && *f != "-Werror").map(String::from).collect(),
        sources,
        run,
    })
}

struct Outcome2 {
    res: Res,
    diags: Vec<Diag>,
    /// The run of a `run` or `run-fail` test, and a line that describes it.
    run: Option<(RunRes, String)>,
}

fn run_one(t: &Test) -> Outcome2 {
    let mut files = vec![t.path.clone()];
    files.extend(t.sources.iter().cloned());
    let mut any_reject = None;
    let mut unsup = Vec::new();
    let want_run = t.run.is_some();
    let (m32, werror, wflags) = (t.m32, t.werror, t.wflags.clone());
    let depth = t.depth;
    let (tx, rx) = mpsc::channel();
    let paths = files.clone();
    // One thread parses every unit, checks each on its own, and runs the
    // linked program if every unit is accepted.
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(move || {
            if let Some(n) = depth {
                crate::types::DEPTH_LIMIT.with(|c| c.set(n));
            }
            let r = std::panic::catch_unwind(|| {
                let ps: Vec<_> = paths
                    .iter()
                    .map(|f| {
                        let src = std::fs::read_to_string(f).unwrap_or_default();
                        parse_source(&src, Some(f), m32, werror, &wflags)
                    })
                    .collect();
                let outs: Vec<Outcome> = ps.iter().map(|p| outcome_of(p).0).collect();
                let ran = if want_run && outs.iter().all(|o| matches!(o, Outcome::Accept)) {
                    // A panic in the interpreter is unsupported, not a
                    // failed check.
                    let refs: Vec<&crate::parser::P> = ps.iter().collect();
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_explored(&refs)));
                    Some(r.unwrap_or((RunOutcome::Unsupported("interpreter crashed".into(), 0), Vec::new(), None)))
                } else {
                    None
                };
                (outs, ran)
            });
            let _ = tx.send(r.map_err(|_| ()));
        })
        .unwrap();
    let mut run_out: Option<(RunOutcome, Vec<RunOutcome>, Option<String>)> = None;
    match rx.recv_timeout(Duration::from_secs(30)) {
        Ok(Ok((outs, ran))) => {
            run_out = ran;
            for o in outs {
                match o {
                    Outcome::Accept => {}
                    Outcome::Reject(d) => {
                        if any_reject.is_none() {
                            any_reject = Some(d);
                        }
                    }
                    Outcome::Unsupported(d) => unsup.extend(d),
                }
            }
        }
        Ok(Err(())) => unsup.push(Diag { sev: crate::diag::Severity::Unsupported, rule: None, cause: None, msg: "front end crashed".into(), line: 0 }),
        Err(_) => unsup.push(Diag { sev: crate::diag::Severity::Unsupported, rule: None, cause: None, msg: "front end timed out".into(), line: 0 }),
    }
    let run = t.run.map(|k| judge_runs(t, k, run_out.as_ref()));
    // The answer is judged by the base expectation. A test whose EDG
    // override differs has a second answer that agrees with EDG.
    let judge = |reject: bool| -> Res {
        if t.no_base {
            Res::NoBase
        } else if reject == t.expect_reject {
            if t.edg_reject.is_some() { Res::EdgOnly } else { Res::Pass }
        } else if t.edg_reject == Some(reject) {
            if t.edg_allowed { Res::EdgAllowed } else { Res::EdgAgrees }
        } else if reject {
            Res::FalseReject
        } else {
            Res::MissedReject
        }
    };
    if let Some(d) = any_reject {
        return Outcome2 { res: judge(true), diags: d, run };
    }
    if !unsup.is_empty() {
        return Outcome2 { res: Res::Unsupported, diags: unsup, run };
    }
    Outcome2 { res: judge(false), diags: vec![], run }
}

/// Compare a run and the other runs that the exploration of its choice
/// points made with the test's expectation (decisions 0047, 0048). Every
/// explored outcome must be one that the expectation allows. A run that
/// stops at undefined behavior that the test expects allows any other
/// outcome.
fn judge_runs(t: &Test, k: RunExpect, o: Option<&(RunOutcome, Vec<RunOutcome>, Option<String>)>) -> (RunRes, String) {
    let Some((first, others, partial)) = o else {
        return judge_run(t, k, None);
    };
    let (res, line) = judge_run(t, k, Some(first));
    let explored = others.len() + 1;
    if res != RunRes::Pass || matches!(first, RunOutcome::Undefined(..)) {
        return (res, line);
    }
    for x in others {
        if matches!(x, RunOutcome::Unsupported(..)) {
            continue;
        }
        let (r, l) = judge_run(t, k, Some(x));
        if r != RunRes::Pass {
            return (RunRes::OtherOutcome, format!("{}; explored: {} runs; an outcome the expectation does not allow: {}", line, explored, l));
        }
    }
    if let Some(why) = partial {
        return (RunRes::PassPartial, format!("{}; explored: {} runs, partial: {}", line, explored, why));
    }
    if let Some(u) = others.iter().find(|x| matches!(x, RunOutcome::Unsupported(..))) {
        return (RunRes::PassPartial, format!("{}; explored: {} runs, one of them {}", line, explored, crate::describe_run(u)));
    }
    if explored > 1 {
        return (res, format!("{}; explored: {} runs", line, explored));
    }
    (res, line)
}

/// Compare a run with the test's expectation (decision 0048).
fn judge_run(t: &Test, k: RunExpect, o: Option<&RunOutcome>) -> (RunRes, String) {
    let Some(o) = o else {
        return (RunRes::NotRun, "the check did not accept the program".to_string());
    };
    let line = crate::describe_run(o);
    let res = match (k, o) {
        (_, RunOutcome::Unsupported(..)) => RunRes::Unsupported,
        // TR-11: the environment receives the value modulo 256.
        (RunExpect::Run, RunOutcome::Result(v, _)) if v.rem_euclid(256) == 0 => RunRes::Pass,
        (RunExpect::Run, RunOutcome::Result(..)) => RunRes::WrongResult,
        (RunExpect::Run, RunOutcome::Undefined(..)) => RunRes::UnexpectedUndefined,
        (RunExpect::RunFail, RunOutcome::Result(v, _)) if v.rem_euclid(256) == 0 => RunRes::MissedUndefined,
        (RunExpect::RunFail, RunOutcome::Result(..)) => RunRes::Pass,
        // The program ends with a nonzero status.
        (RunExpect::Run, RunOutcome::Terminated(..)) => RunRes::WrongResult,
        (RunExpect::RunFail, RunOutcome::Terminated(..)) => RunRes::Pass,
        // Only a stop at undefined behavior meets an interpreter-only
        // expectation.
        (RunExpect::Undefined, RunOutcome::Result(..) | RunOutcome::Terminated(..)) => RunRes::MissedUndefined,
        (RunExpect::RunFail | RunExpect::Undefined, RunOutcome::Undefined(r, ..)) => {
            // A stop can name several rules, separated by commas.
            if r.split(", ").any(|r| t.rules.iter().any(|x| x == r)) {
                RunRes::Pass
            } else {
                RunRes::UnexpectedUndefined
            }
        }
    };
    (res, line)
}

/// The status of a test that expects a run, after the check accepted
/// it: `pass`, or the way the run failed.
fn run_tag(rr: &RunRes) -> &'static str {
    match rr {
        RunRes::Pass => "pass",
        RunRes::PassPartial => "pass-partial",
        RunRes::OtherOutcome => "RUN-OTHER-OUTCOME",
        RunRes::WrongResult => "RUN-WRONG-RESULT",
        RunRes::MissedUndefined => "RUN-MISSED-UB",
        RunRes::UnexpectedUndefined => "RUN-UNEXPECTED-UB",
        RunRes::Unsupported => "run-unsupported",
        RunRes::NotRun => "run-not-run",
    }
}

fn normalize(msg: &str) -> String {
    // collapse names and numbers so that similar messages group together
    let mut out = String::new();
    let mut in_quote = false;
    for c in msg.chars() {
        if c == '\'' {
            in_quote = !in_quote;
            if in_quote {
                out.push_str("'_'");
            }
            continue;
        }
        if in_quote {
            continue;
        }
        if c.is_ascii_digit() {
            if !out.ends_with('N') {
                out.push('N');
            }
            continue;
        }
        out.push(c);
    }
    out
}

pub fn run(args: &[String]) {
    let root = PathBuf::from(&args[0]);
    let verbose = args.iter().any(|a| a == "-v");
    let out_file = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)).cloned();
    let mut paths = Vec::new();
    collect(&root, &mut paths);
    // The rule kinds come from the specification beside the tests, or from
    // the directory that `--spec` names.
    let spec = args.iter().position(|a| a == "--spec").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(|| root.join("..").join("specification"));
    let spec_rules = read_rules(&spec);
    let kinds = &spec_rules.kinds;
    if kinds.is_empty() {
        eprintln!("conform: no rules found under {}; no test counts as allowing EDG's answer", spec.display());
    }
    let tests: Vec<Test> = paths.iter().filter_map(|p| parse_test(&root, p, kinds)).collect();
    // run in parallel
    let n = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4);
    let tests = std::sync::Arc::new(tests);
    let next = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..n {
        let tests = tests.clone();
        let next = next.clone();
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || loop {
            let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if i >= tests.len() {
                break;
            }
            let o = run_one(&tests[i]);
            tx.send((i, o)).unwrap();
        }));
    }
    drop(tx);
    let mut results: Vec<Option<Outcome2>> = (0..tests.len()).map(|_| None).collect();
    for (i, o) in rx {
        results[i] = Some(o);
    }
    for h in handles {
        let _ = h.join();
    }
    // tally
    let mut by_dir: BTreeMap<String, [usize; 9]> = BTreeMap::new();
    let mut by_sub: BTreeMap<String, [usize; 9]> = BTreeMap::new();
    // Per page: the passing rejects, those whose first error names a rule
    // the test cites, those whose first error names a rule that the text
    // of a cited rule links to, and those whose first error names no rule.
    let mut by_page: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut rule_lines = Vec::new();
    let mut unsup_msgs: BTreeMap<String, usize> = BTreeMap::new();
    let mut rule_match = 0;
    let mut lines = Vec::new();
    for (t, o) in tests.iter().zip(results.iter()) {
        let o = o.as_ref().unwrap();
        let parts: Vec<&str> = t.rel.split('/').collect();
        let top = parts.first().copied().unwrap_or("").to_string();
        let sub = if parts.len() > 2 { format!("{}/{}", parts[0], parts[1]) } else { top.clone() };
        let idx = match o.res {
            Res::Pass => 0,
            Res::FalseReject => 1,
            Res::MissedReject => 2,
            Res::Unsupported => 3,
            Res::EdgOnly => 5,
            Res::EdgAgrees => 6,
            Res::EdgAllowed => 7,
            Res::NoBase => 8,
        };
        for k in [&mut by_dir.entry(top).or_default(), &mut by_sub.entry(sub.clone()).or_default()] {
            k[idx] += 1;
            k[4] += 1;
        }
        if o.res == Res::Unsupported {
            let mut seen = std::collections::HashSet::new();
            for d in &o.diags {
                let m = normalize(&d.msg);
                if seen.insert(m.clone()) {
                    *unsup_msgs.entry(m).or_default() += 1;
                }
            }
        }
        if matches!(o.res, Res::Pass | Res::EdgOnly) && t.expect_reject {
            if o.diags.iter().any(|d| d.rule.into_iter().chain(d.cause).any(|r| t.rules.iter().any(|x| x == r))) {
                rule_match += 1;
            }
            // The first error and the rules it names: the rule, and the
            // rule that a constant evaluation broke.
            let e = by_page.entry(sub.clone()).or_default();
            e[0] += 1;
            let first = o.diags.iter().find(|d| d.sev == crate::diag::Severity::Error);
            let named: Vec<&str> = first.map(|d| d.rule.into_iter().chain(d.cause).collect()).unwrap_or_default();
            let linked = |r: &str| t.rules.iter().any(|x| spec_rules.links.get(x).is_some_and(|l| l.iter().any(|y| y == r)));
            if named.iter().any(|r| t.rules.iter().any(|x| x == r)) {
                e[1] += 1;
            } else if named.iter().any(|r| linked(r)) {
                e[2] += 1;
            } else if named.is_empty() {
                e[3] += 1;
                rule_lines.push(format!("RULE-ID\tno-rule\t{}\t{}", t.rel, t.rules.join(" ")));
            } else {
                rule_lines.push(format!("RULE-ID\tother-rule\t{}\t{}\t{}", t.rel, t.rules.join(" "), named.join(" ")));
            }
        }
        let mut tag = match o.res {
            Res::Pass => "pass",
            Res::FalseReject => "FALSE-REJECT",
            Res::MissedReject => "MISSED-REJECT",
            Res::Unsupported => "unsupported",
            Res::EdgOnly => "EDG-ONLY",
            Res::EdgAgrees => "EDG-AGREES",
            Res::EdgAllowed => "EDG-ALLOWED",
            Res::NoBase => "NO-BASE",
        };
        // A test that expects a run passes only if the run passes too. The
        // status column then gives the run's outcome.
        let mut first = o.diags.first().map(|d| format!("{}:{} {}", d.line, d.rule.unwrap_or("-"), d.msg)).unwrap_or_default();
        if matches!(o.res, Res::Pass | Res::EdgOnly) {
            if let Some((rr, line)) = &o.run {
                let rt = run_tag(rr);
                if rt != "pass" && *rr != RunRes::NotRun {
                    tag = rt;
                    first = line.clone();
                }
            }
        }
        lines.push(format!("{}\t{}\t{}\t{}", tag, t.rel, t.rules.join(" "), first));
    }
    // The run tests: a table of their own.
    let mut run_dir: BTreeMap<String, [usize; 9]> = BTreeMap::new();
    let mut run_lines = Vec::new();
    for (t, o) in tests.iter().zip(results.iter()) {
        let o = o.as_ref().unwrap();
        let Some((rr, line)) = &o.run else { continue };
        let idx = match rr {
            RunRes::Pass => 0,
            RunRes::WrongResult => 1,
            RunRes::MissedUndefined => 2,
            RunRes::UnexpectedUndefined => 3,
            RunRes::Unsupported => 4,
            RunRes::NotRun => 5,
            RunRes::OtherOutcome => 7,
            RunRes::PassPartial => 8,
        };
        let parts: Vec<&str> = t.rel.split('/').collect();
        let top = parts.first().copied().unwrap_or("").to_string();
        let e = run_dir.entry(top).or_default();
        e[idx] += 1;
        e[6] += 1;
        if *rr == RunRes::PassPartial {
            // A partial pass is a pass, counted again in its own column.
            e[0] += 1;
        }
        let tag = match rr {
            RunRes::Pass => "run-pass",
            RunRes::PassPartial => "run-pass-partial",
            RunRes::OtherOutcome => "RUN-OTHER-OUTCOME",
            RunRes::WrongResult => "RUN-WRONG-RESULT",
            RunRes::MissedUndefined => "RUN-MISSED-UB",
            RunRes::UnexpectedUndefined => "RUN-UNEXPECTED-UB",
            RunRes::Unsupported => "run-unsupported",
            RunRes::NotRun => "run-not-run",
        };
        // The tag comes second, so that tools that read the check lines
        // (tools/frontend_check.py) skip these lines.
        run_lines.push(format!("RUN\t{}\t{}\t{}\t{}", tag, t.rel, t.rules.join(" "), line));
    }
    let mut report = String::new();
    // The check is judged by the base expectation. "Differs from EDG
    // only" is a pass; "agrees with EDG only" is a failure; "EDG, allowed"
    // is a pass on a test whose rules allow EDG's answer. "No base
    // expectation" is a failure: the test gives no answer to judge by.
    const HEAD: &str = "| Directory | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only | Agrees with EDG only | EDG, allowed | No base expectation |\n|---|---|---|---|---|---|---|---|---|---|\n";
    let row = |d: &str, c: &[usize; 9]| format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", d, c[4], c[0], c[1], c[2], c[3], c[5], c[6], c[7], c[8]);
    report.push_str(HEAD);
    let mut tot = [0usize; 9];
    for (d, c) in &by_dir {
        report.push_str(&row(d, c));
        for i in 0..9 {
            tot[i] += c[i];
        }
    }
    report.push_str(&row("**total**", &tot));
    report.push_str(&format!("\nRejected tests where the diagnostic names one of the test's rules: {}\n", rule_match));
    // The rule IDs of the first error, per page (roadmap round 6). A rule
    // that the text of a cited rule links to also counts, as decision 0052
    // lets a run stop match a linked rule.
    report.push_str("\nPassing rejects whose first error names a rule, by page:\n\n| Page | Passing rejects | A cited rule | A linked rule | Another rule | No rule | Share |\n|---|---|---|---|---|---|---|\n");
    let mut pt = [0usize; 4];
    let share = |c: &[usize; 4]| if c[0] == 0 { "-".to_string() } else { format!("{}%", (100 * (c[1] + c[2]) + c[0] / 2) / c[0]) };
    for (d, c) in &by_page {
        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", d, c[0], c[1], c[2], c[0] - c[1] - c[2] - c[3], c[3], share(c)));
        for i in 0..4 {
            pt[i] += c[i];
        }
    }
    report.push_str(&format!("| **total** | {} | {} | {} | {} | {} | {} |\n", pt[0], pt[1], pt[2], pt[0] - pt[1] - pt[2] - pt[3], pt[3], share(&pt)));
    report.push_str("\nBy subdirectory:\n\n");
    report.push_str(HEAD);
    for (d, c) in &by_sub {
        report.push_str(&row(d, c));
    }
    // "Pass, partial" counts the passes whose exploration of choice points
    // the bound stopped or an unsupported run left partial; they are in
    // "Pass" too. "Other outcome" counts the runs whose first outcome is
    // allowed and another explored outcome is not.
    report.push_str("\nRun tests (`run` and `run-fail`):\n\n| Directory | Tests | Pass | Pass, partial | Other outcome | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |\n|---|---|---|---|---|---|---|---|---|---|\n");
    let mut rt = [0usize; 9];
    for (d, c) in &run_dir {
        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", d, c[6], c[0], c[8], c[7], c[1], c[2], c[3], c[4], c[5]));
        for i in 0..9 {
            rt[i] += c[i];
        }
    }
    report.push_str(&format!("| **total** | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", rt[6], rt[0], rt[8], rt[7], rt[1], rt[2], rt[3], rt[4], rt[5]));
    let mut run_unsup: BTreeMap<String, usize> = BTreeMap::new();
    for l in &run_lines {
        if let Some(i) = l.find("unsupported at line ") {
            let rest = &l[i..];
            let msg = rest.split_once(": ").map(|x| x.1).unwrap_or(rest);
            *run_unsup.entry(normalize(msg)).or_default() += 1;
        } else if l.contains("\trun-unsupported\t") {
            *run_unsup.entry(l.rsplit('\t').next().unwrap_or("").to_string()).or_default() += 1;
        }
    }
    let mut ru: Vec<_> = run_unsup.into_iter().collect();
    ru.sort_by(|a, b| b.1.cmp(&a.1));
    report.push_str("\nConstructs the interpreter does not run (tests affected):\n\n");
    for (m, c) in ru.iter().take(if verbose { 400 } else { 30 }) {
        report.push_str(&format!("  {:5}  {}\n", c, m));
    }
    let mut um: Vec<_> = unsup_msgs.into_iter().collect();
    um.sort_by(|a, b| b.1.cmp(&a.1));
    report.push_str("\nUnsupported constructs (tests affected):\n\n");
    for (m, c) in um.iter().take(if verbose { 400 } else { 40 }) {
        report.push_str(&format!("  {:5}  {}\n", c, m));
    }
    print!("{}", report);
    if verbose {
        for l in &lines {
            if !l.starts_with("pass\t") {
                println!("{}", l);
            }
        }
        for l in &run_lines {
            if !l.starts_with("RUN\trun-pass\t") {
                println!("{}", l);
            }
        }
        // The tag comes second here too, so tools/frontend_check.py skips
        // these lines.
        for l in &rule_lines {
            println!("{}", l);
        }
    }
    if let Some(f) = out_file {
        let mut s = report.clone();
        s.push_str("\n\nPer test:\n");
        for l in &lines {
            s.push_str(l);
            s.push('\n');
        }
        for l in &run_lines {
            s.push_str(l);
            s.push('\n');
        }
        for l in &rule_lines {
            s.push_str(l);
            s.push('\n');
        }
        let _ = std::fs::write(f, s);
    }
}
