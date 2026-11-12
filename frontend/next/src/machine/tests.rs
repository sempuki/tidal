//! Unit tests of the machine: whole programs through analysis and the
//! run, and constant evaluation with its limits.

use crate::foundations::translation::run::{run_program, RunOutcome};
use crate::Outcome;
use std::path::Path;

fn unit(src: &str) -> crate::sem::Unit {
    let (o, u) = crate::check_source(src, Path::new("t.cpp"), &crate::Options::default());
    assert!(matches!(o, Outcome::Accept), "the unit is not accepted");
    u.unwrap()
}

fn run(src: &str) -> RunOutcome {
    run_program(&unit(src))
}

#[test]
fn main_returns_its_status() {
    assert_eq!(run("int main() { return 3; }"), RunOutcome::Result(3));
    assert_eq!(run("int main() {}"), RunOutcome::Result(0));
}

#[test]
fn loops_and_jumps_run() {
    let src = "int main() { int s = 0; for (int i = 0; i < 10; ++i) { if (i % 2) continue; if (i > 6) break; s += i; } \
               int k = 0; while (k < 3) k++; do { k += 10; } while (k < 20); return s * 100 + k; }";
    assert_eq!(run(src), RunOutcome::Result(12 * 100 + 23));
}

#[test]
fn switch_and_goto_reach_their_labels() {
    let src = "int f(int v) { switch (v) { case 1: return 10; case 2: v = 7; default: return v; } } \
               int main() { int n = 0; again: ++n; if (n < 3) goto again; return f(1) + f(2) + n; }";
    assert_eq!(run(src), RunOutcome::Result(10 + 7 + 3));
}

#[test]
fn undefined_behavior_names_its_rule() {
    let r = run("int main() { int a[3] = {1, 2, 3}; return a[3]; }");
    assert!(matches!(r, RunOutcome::Undefined("PFX-4", ..)), "{:?}", r);
    let r = run("int main() { int z = 0; return 1 / z; }");
    assert!(matches!(r, RunOutcome::Undefined("MUL-9", ..)), "{:?}", r);
    let r = run("int main() { int x; return x; }");
    assert!(matches!(r, RunOutcome::Undefined("INIT-1", ..)), "{:?}", r);
    let r = run("int main() { int m = 2147483647; return m + 1 > 0; }");
    assert!(matches!(r, RunOutcome::Undefined("ADD-9", ..)), "{:?}", r);
}

#[test]
fn pointers_move_inside_their_arrays() {
    let src = "int main() { int a[4] = {1, 2, 3, 4}; int *p = a; p += 2; int *e = a + 4; return *p + int(e - p); }";
    assert_eq!(run(src), RunOutcome::Result(3 + 2));
}

#[test]
fn constant_evaluation_reports_its_limits() {
    let (o, u) = crate::check_source("int n = 3; static_assert(n == 3);", Path::new("t.cpp"), &crate::Options::default());
    assert!(matches!(o, Outcome::Reject(_)));
    let d = u.unwrap().diags.into_iter().find(|d| d.sev == crate::diag::Severity::Error).unwrap();
    assert_eq!((d.rule, d.cause), (Some("SA-7"), Some("CE-3")));
    unit("constexpr int sq(int x) { return x * x; } static_assert(sq(4) == 16); const int k = 5; int a[k];");
}
