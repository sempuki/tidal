// Conformance harness: runs every test under a directory and compares the
// front end's answer with the test's expectation.

use crate::diag::Diag;
use crate::{check_source, Outcome};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

struct Test {
    path: PathBuf,
    rel: String,
    expect_reject: bool,
    /// The expectation without the EDG override, when there is one.
    base_reject: Option<bool>,
    rules: Vec<String>,
    m32: bool,
    werror: bool,
    /// The -W flags other than -Werror.
    wflags: Vec<String>,
    sources: Vec<PathBuf>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Res {
    Pass,
    FalseReject,
    MissedReject,
    Unsupported,
    /// Disagrees with the EDG override, agrees with the base expectation.
    EdgOnly,
}

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

fn parse_test(root: &Path, path: &Path) -> Option<Test> {
    let src = std::fs::read_to_string(path).ok()?;
    let mut expect = None;
    let mut edg = None;
    let mut rules = Vec::new();
    let mut flags = String::new();
    let mut sources = Vec::new();
    for line in src.lines() {
        let Some(rest) = line.strip_prefix("// tidal-") else {
            if !line.starts_with("//") {
                break;
            }
            continue;
        };
        if let Some(v) = rest.strip_prefix("expect:") {
            expect = Some(v.trim().to_string());
        } else if let Some(v) = rest.strip_prefix("expect-edg:") {
            edg = Some(v.trim().to_string());
        } else if let Some(v) = rest.strip_prefix("rules:") {
            rules = v.split_whitespace().map(String::from).collect();
        } else if let Some(v) = rest.strip_prefix("flags:") {
            flags = v.trim().to_string();
        } else if let Some(v) = rest.strip_prefix("sources:") {
            for s in v.split_whitespace() {
                sources.push(path.parent().unwrap().join(s));
            }
        }
    }
    // A front end only checks programs: run, run-fail, and link-fail mean
    // accept. An EDG override takes precedence.
    let base_reject = if edg.is_some() { expect.as_ref().map(|x| x == "reject") } else { None };
    let e = edg.or(expect)?;
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
    Some(Test {
        path: path.to_path_buf(),
        rel,
        expect_reject: e == "reject",
        base_reject,
        rules,
        m32: flags.contains("-m32"),
        werror: flags.contains("-Werror"),
        wflags: flags.split_whitespace().filter(|f| f.starts_with("-W") && *f != "-Werror").map(String::from).collect(),
        sources,
    })
}

struct Outcome2 {
    res: Res,
    diags: Vec<Diag>,
}

fn run_one(t: &Test) -> Outcome2 {
    let mut files = vec![t.path.clone()];
    files.extend(t.sources.iter().cloned());
    let mut any_reject = None;
    let mut unsup = Vec::new();
    for f in files {
        let src = std::fs::read_to_string(&f).unwrap_or_default();
        let (m32, werror, wflags) = (t.m32, t.werror, t.wflags.clone());
        let (tx, rx) = mpsc::channel();
        let fpath = f.clone();
        std::thread::Builder::new()
            .stack_size(64 << 20)
            .spawn(move || {
                let r = std::panic::catch_unwind(|| check_source(&src, Some(&fpath), m32, werror, &wflags));
                let _ = tx.send(r.map(|(o, _)| o).map_err(|_| ()));
            })
            .unwrap();
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(Ok(Outcome::Accept)) => {}
            Ok(Ok(Outcome::Reject(d))) => {
                if any_reject.is_none() {
                    any_reject = Some(d);
                }
            }
            Ok(Ok(Outcome::Unsupported(d))) => unsup.extend(d),
            Ok(Err(())) => unsup.push(Diag { sev: crate::diag::Severity::Unsupported, rule: None, cause: None, msg: "front end crashed".into(), line: 0 }),
            Err(_) => unsup.push(Diag { sev: crate::diag::Severity::Unsupported, rule: None, cause: None, msg: "front end timed out".into(), line: 0 }),
        }
    }
    if let Some(d) = any_reject {
        let res = if t.expect_reject {
            Res::Pass
        } else if t.base_reject == Some(true) {
            Res::EdgOnly
        } else {
            Res::FalseReject
        };
        return Outcome2 { res, diags: d };
    }
    if !unsup.is_empty() {
        return Outcome2 { res: Res::Unsupported, diags: unsup };
    }
    let res = if !t.expect_reject {
        Res::Pass
    } else if t.base_reject == Some(false) {
        Res::EdgOnly
    } else {
        Res::MissedReject
    };
    Outcome2 { res, diags: vec![] }
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
    let tests: Vec<Test> = paths.iter().filter_map(|p| parse_test(&root, p)).collect();
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
    let mut by_dir: BTreeMap<String, [usize; 6]> = BTreeMap::new();
    let mut by_sub: BTreeMap<String, [usize; 6]> = BTreeMap::new();
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
        };
        for k in [&mut by_dir.entry(top).or_default(), &mut by_sub.entry(sub).or_default()] {
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
        if o.res == Res::Pass && t.expect_reject {
            if o.diags.iter().any(|d| d.rule.into_iter().chain(d.cause).any(|r| t.rules.iter().any(|x| x == r))) {
                rule_match += 1;
            }
        }
        let tag = match o.res {
            Res::Pass => "pass",
            Res::FalseReject => "FALSE-REJECT",
            Res::MissedReject => "MISSED-REJECT",
            Res::Unsupported => "unsupported",
            Res::EdgOnly => "EDG-ONLY",
        };
        let first = o.diags.first().map(|d| format!("{}:{} {}", d.line, d.rule.unwrap_or("-"), d.msg)).unwrap_or_default();
        lines.push(format!("{}\t{}\t{}\t{}", tag, t.rel, t.rules.join(" "), first));
    }
    let mut report = String::new();
    report.push_str("| Directory | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only |\n|---|---|---|---|---|---|---|\n");
    let mut tot = [0usize; 6];
    for (d, c) in &by_dir {
        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", d, c[4], c[0], c[1], c[2], c[3], c[5]));
        for i in 0..6 {
            tot[i] += c[i];
        }
    }
    report.push_str(&format!("| **total** | {} | {} | {} | {} | {} | {} |\n", tot[4], tot[0], tot[1], tot[2], tot[3], tot[5]));
    report.push_str(&format!("\nRejected tests where the diagnostic names one of the test's rules: {}\n", rule_match));
    report.push_str("\nBy subdirectory:\n\n| Directory | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only |\n|---|---|---|---|---|---|---|\n");
    for (d, c) in &by_sub {
        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", d, c[4], c[0], c[1], c[2], c[3], c[5]));
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
            if !l.starts_with("pass") {
                println!("{}", l);
            }
        }
    }
    if let Some(f) = out_file {
        let mut s = report.clone();
        s.push_str("\n\nPer test:\n");
        for l in &lines {
            s.push_str(l);
            s.push('\n');
        }
        let _ = std::fs::write(f, s);
    }
}
