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

#[test]
fn an_error_after_an_unmodelled_construct_gives_no_verdict() {
    let outcome = |src: &str| crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0;
    assert!(matches!(outcome("struct S { int a; };\nstatic_assert(1 == 2);"), Outcome::Unsupported(_)));
    assert!(matches!(outcome("static_assert(1 == 2);\nstruct S { int a; };"), Outcome::Reject(_)));
    assert!(matches!(outcome("int int::* pm;"), Outcome::Reject(_)));
}

#[test]
fn unsequenced_accesses_and_volatile_objects() {
    let rule = |src: &str| match run(src) {
        RunOutcome::Undefined(r, ..) => Some(r),
        _ => None,
    };
    assert_eq!(rule("int main() { int i = 2; return i++ * i; }"), Some("SEQ-4"));
    assert_eq!(rule("int main() { int i = 2; return i << i++; }"), None);
    assert_eq!(rule("int f(int& i) { return ++i; } int main() { int i = 0; return f(i) + i; }"), None);
    assert_eq!(rule("volatile int v = 3; int main() { int* p = const_cast<int*>(&v); return *p; }"), Some("VOL-6"));
    assert_eq!(rule("volatile int a[] = {1, 2}; int main() { int s = 0; for (int x : a) s += x; return s; }"), None);
}

#[test]
fn casts_to_enumerations_stay_in_their_values() {
    let rule = |src: &str| match run(src) {
        RunOutcome::Undefined(r, ..) => Some(r),
        r => {
            assert!(matches!(r, RunOutcome::Result(_)), "{:?}", r);
            None
        }
    };
    assert_eq!(rule("enum C { a, b, c }; int main() { int n = 4; C x = static_cast<C>(n); return x == a; }"), Some("ENUM-6"));
    assert_eq!(rule("enum C { a, b, c }; int main() { int n = 3; C x = static_cast<C>(n); return x == a; }"), None);
    assert_eq!(rule("enum E {}; int main() { int n = 1; E x = static_cast<E>(n); return 0; }"), None);
    assert_eq!(rule("enum F : unsigned char { z }; int main() { int n = 300; F x = static_cast<F>(n); return x == z; }"), None);
}

#[test]
fn a_reference_keeps_only_the_temporary_it_binds() {
    let rule = |src: &str| match run(src) {
        RunOutcome::Undefined(r, ..) => Some(r),
        _ => None,
    };
    assert_eq!(rule("const int& pass(const int& x) { return x; } int main() { const int& r = pass(3); return r; }"), Some("OBJ-2"));
    assert_eq!(rule("int main() { int n = 2; const int& r = n + 1; return r - 3; }"), None);
    assert_eq!(rule("int main() { int n = 2; const int& r = n > 1 ? 4 : 5; return r - 4; }"), None);
    assert_eq!(rule("int f(int v) { return v; } int main() { int (&g)(int) = f; return g(0); }"), None);
}

#[test]
fn a_c_function_in_two_namespaces_is_one_candidate() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    let unit = |d1: &str, d2: &str, call: &str| {
        format!("namespace a {{ extern \"C\" void f(int = {d1}); }} namespace b {{ extern \"C\" void f(int = {d2}); }} using a::f; using b::f; void g() {{ {call}; }}")
    };
    assert_eq!(rule(&unit("5", "(0x5)", "f()")), None);
    assert_eq!(rule(&unit("5", "7", "f(1)")), None);
    assert_eq!(rule(&unit("5", "7", "f()")), Some("OVL-38"));
    assert_eq!(rule(&unit("5", "2 + 3", "f()")), Some("OVL-38"));
}

#[test]
fn an_alias_names_a_qualified_function_type() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("using F = void() const; typedef int G() &&;"), None);
    assert_eq!(rule("using F = void() const; F* p;"), Some("DCL-16"));
    assert_eq!(rule("using F = void() &; void take(F& f);"), Some("DCL-16"));
    assert_eq!(rule("using F = void() const; F f;"), Some("DCL-21"));
    assert_eq!(rule("void f() const;"), Some("DCL-21"));
}

#[test]
fn lookup_in_enumerations_arguments_and_qualified_declarators() {
    let accepts = |src: &str| matches!(crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0, Outcome::Accept);
    assert!(accepts("constexpr int v = 1; enum class E { v = v + 1, w = v * 2 }; static_assert(int(E::w) == 4);"));
    assert!(accepts("namespace n { enum class m { on }; int probe(m) { return 1; } } int r() { return probe(n::m::on); }"));
    assert!(!accepts("namespace n { int twice(int x) { return x; } } int r() { return twice(1); }"));
    assert!(accepts("namespace N { constexpr int k = 2; extern int a[k]; } int N::a[k];"));
}

#[test]
fn attributes_on_using_directives_and_qualified_reference_bindings() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("namespace t { int c; } [[]] using namespace t; int f() { [[]] using namespace t; return c; }"), None);
    assert_eq!(rule("namespace t { int c; } [[deprecated]] using namespace t;"), Some("ATTR-2"));
    assert_eq!(rule("int* m(); int p(int* const&); int p(const int* const&); int r = p(m());"), None);
}

#[test]
fn comparison_warnings() {
    let rule = |src: &str, flags: &[&str]| {
        let o = crate::Options { werror: true, wflags: flags.iter().map(|f| f.to_string()).collect(), ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    assert_eq!(rule("bool f(int a, int b, int c) { return a < b < c; }", &["-Wparentheses"]), Some("CMP-14"));
    assert_eq!(rule("bool f(int a, int b, int c) { return (a < b) < c; }", &["-Wparentheses"]), None);
    assert_eq!(rule("int a[2], b[2]; bool f() { return a == b; }", &["-Wall"]), Some("CMP-16"));
    assert_eq!(rule("bool f(int a, unsigned b) { return a < b; }", &["-Wsign-compare"]), Some("CMP-4"));
    assert_eq!(rule("bool f(unsigned b) { return 0 < b; }", &["-Wsign-compare"]), None);
    assert_eq!(rule("enum E { a }; bool f(double d) { return a < d; }", &[]), Some("CMP-21"));
}

#[test]
fn deduced_return_types() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("auto f(bool c) { if (c) return 1; return 2; } int x = f(true);"), None);
    assert_eq!(rule("auto f(bool c) { if (c) return 1; return 2.0; }"), Some("RET-7"));
    assert_eq!(rule("auto f() { if constexpr (false) { return 1.5; } else { return 2; } } int (*p)() = f;"), None);
    assert_eq!(rule("auto f();\nint x = f();\nauto f() { return 1; }"), Some("RET-3"));
    assert_eq!(rule("auto f(int n) { if (n > 0) return f(n - 1); return 0; }"), Some("RET-3"));
    assert_eq!(rule("auto f(int n) { if (n == 0) return 0; return f(n - 1); }"), None);
    assert_eq!(rule("auto f();\nint f();"), Some("RET-4"));
    assert_eq!(rule("auto& f() {}"), Some("RET-3"));
    assert_eq!(rule("auto& f() { static int x; return x; }"), Some("unsupported"));
}

#[test]
fn flowing_off_the_end_of_a_function() {
    let warns = |src: &str| {
        let o = crate::Options { werror: true, wflags: vec!["-Wreturn-type".into()], ..Default::default() };
        matches!(crate::check_source(src, Path::new("t.cpp"), &o).0, Outcome::Reject(ds) if ds[0].rule == Some("FN-7"))
    };
    assert!(warns("int f(bool b) { if (b) return 1; }"));
    assert!(!warns("int f(bool b) { if (b) return 1; else return 2; }"));
    assert!(!warns("int f() { while (true) {} }"));
    assert!(warns("extern bool d; int f() { while (true) { if (d) break; } }"));
    assert!(!warns("extern bool d; int f() { while (true) { for (;;) { break; } } }"));
    assert!(warns("int f(int n) { switch (n) { case 0: return 1; } }"));
    assert!(!warns("int f(int n) { switch (n) { case 0: return 1; default: return 2; } }"));
    assert!(!warns("[[noreturn]] void stop(); int f() { stop(); }"));
    assert!(!warns("int main() {}"));
}

#[test]
fn a_discarded_volatile_glvalue_is_read_in_the_listed_forms() {
    let rule = |body: &str| {
        let src = format!("constexpr int f() {{ volatile int v = 1; volatile int w = 2; {body} return 0; }} static_assert(f() == 0);");
        match crate::check_source(&src, Path::new("t.cpp"), &crate::Options::default()).0 {
            Outcome::Reject(ds) => ds[0].cause,
            _ => None,
        }
    };
    assert_eq!(rule("v;"), Some("CE-12"));
    assert_eq!(rule("true ? v : w;"), Some("CE-12"));
    assert_eq!(rule("(void)0, v;"), Some("CE-12"));
    assert_eq!(rule("v = 3;"), None);
    assert_eq!(rule("(void)&v;"), None);
}

#[test]
fn enumeration_declarations_agree_and_hold_their_values() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("enum class F : short; enum class F : int { A };"), Some("ENUM-18"));
    assert_eq!(rule("enum E : int; enum E { a };"), Some("ENUM-18"));
    assert_eq!(rule("enum class E : long; enum class E { a };"), Some("ENUM-18"));
    assert_eq!(rule("enum class E; enum class E : int { a };"), None);
    assert_eq!(rule("enum class F : int { A }; F f = {5};"), Some("ENUM-19"));
    assert_eq!(rule("enum class F : int { A }; F f{5};"), None);
    assert_eq!(rule("enum H { A = 1ULL << 63, B = -1 };"), Some("ENUM-5"));
}

#[test]
fn warnings_for_values_without_effect_and_mixed_enumerations() {
    let rule = |src: &str, flags: &[&str]| {
        let o = crate::Options { werror: true, wflags: flags.iter().map(|f| f.to_string()).collect(), ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    let w = &["-Wunused-value"];
    assert_eq!(rule("void f(int a) { a + 1; }", w), Some("STMT-4"));
    assert_eq!(rule("int f(int a, int b) { return (a + 1, b); }", w), Some("COMMA-5"));
    assert_eq!(rule("int f(int k, int n) { for (; k < n; n) ++k; return k; }", w), Some("UNEV-3"));
    assert_eq!(rule("int g(); volatile int p; void f(int a) { p; a = 1; ++a; g(); (void)a; a ? g() : g(); }", w), None);
    assert_eq!(rule("enum E { A }; enum F { B }; int f() { return A + B; }", &[]), Some("ADD-16"));
    assert_eq!(rule("enum E { A }; enum F { B }; int f() { return A | B; }", &[]), Some("BIT-8"));
    assert_eq!(rule("enum E { A }; double f() { return A * 1.5; }", &[]), Some("MUL-16"));
    assert_eq!(rule("enum E { A }; double f() { return A | 1; }", &[]), None);
}

#[test]
fn operator_warnings() {
    let rule = |src: &str, flags: &[&str]| {
        let o = crate::Options { werror: true, wflags: flags.iter().map(|f| f.to_string()).collect(), ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    assert_eq!(rule("bool f(bool a, bool b, bool c) { return a || b && c; }", &["-Wparentheses"]), Some("LOG-8"));
    assert_eq!(rule("bool f(bool a, bool b, bool c) { return a || (b && c); }", &["-Wparentheses"]), None);
    assert_eq!(rule("bool f(int a, int b) { return !a == b; }", &["-Wall"]), Some("LOG-9"));
    assert_eq!(rule("bool f(int a, int b) { return (!a) == b; }", &["-Wall"]), None);
    assert_eq!(rule("bool f(int a, int b, int c) { return a & b == c; }", &["-Wparentheses"]), Some("BIT-6"));
    assert_eq!(rule("int f(bool b) { return ~b; }", &["-Wbool-operation"]), Some("BIT-7"));
    assert_eq!(rule("int f() { return 1 << 33; }", &[]), Some("SHF-9"));
    assert_eq!(rule("int f(int v) { return v << -1; }", &[]), Some("SHF-9"));
    assert_eq!(rule("int f(int v) { return v << 31; }", &[]), None);
}

#[test]
fn switch_and_volatile_warnings() {
    let rule = |src: &str| {
        let o = crate::Options { werror: true, wflags: vec!["-Wall".into()], ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    let sw = |cases: &str| format!("enum L {{ r, a, g }}; int c(L l) {{ switch (l) {{ {cases} }} return 0; }}");
    assert_eq!(rule(&sw("case r: return 1; case a: return 2;")), Some("SW-7"));
    assert_eq!(rule(&sw("case r: return 1; default: return 2;")), None);
    assert_eq!(rule(&sw("case r: case a: case g: return 1; case 5: return 2;")), Some("SW-8"));
    assert_eq!(rule("volatile int v; int f() { int a = (v = 1); return a; }"), Some("VOL-5"));
    assert_eq!(rule("volatile int v; void f() { v = 1; v += 2; }"), None);
    assert_eq!(rule("volatile int f();"), Some("CV-9"));
    assert_eq!(rule("void g(volatile int);"), Some("CV-9"));
    assert_eq!(rule("void g(volatile int*); volatile int& h();"), None);
}

#[test]
fn default_arguments_belong_to_function_declarations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("using S = int(int = 1);"), Some("DEFARG-5"));
    assert_eq!(rule("typedef int S(int = 1);"), Some("DEFARG-5"));
    assert_eq!(rule("int (*p)(int a = 1);"), Some("DEFARG-5"));
    assert_eq!(rule("int f(int a = 1);"), None);
    assert_eq!(rule("void g() { int local = 1; int f(int v = local); }"), Some("DEFARG-4"));
    assert_eq!(rule("void g() { int local = 1; int f(int v = sizeof(local)); }"), None);
}

#[test]
fn namespaces_inline_aliases_and_qualified_definitions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("namespace a { int x; } namespace b = a; namespace b { int y; }"), Some("NS-14"));
    assert_eq!(rule("namespace A { inline namespace B { int x; } int x; } int y = A::x;"), Some("NAM-26"));
    assert_eq!(rule("namespace A { inline namespace B { int x; } } int y = A::x;"), None);
    assert_eq!(rule("namespace s { int f(); } namespace g { using namespace s; } int g::f() { return 0; }"), Some("NS-15"));
    assert_eq!(rule("namespace g { namespace { int f(); } } int g::f() { return 0; }"), Some("NS-15"));
    assert_eq!(rule("namespace N {} int N::x = 7;"), Some("NS-15"));
    assert_eq!(rule("namespace N { inline namespace V { extern int x; } } int N::x = 7;"), None);
}

#[test]
fn fallthrough_placement_and_warning() {
    let rule = |src: &str| {
        let o = crate::Options { werror: true, wflags: vec!["-Wimplicit-fallthrough".into()], ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    let sw = |body: &str| format!("int f(int k, bool w) {{ int r = 0; switch (k) {{ {body} }} return r; }}");
    assert_eq!(rule(&sw("case 0: r = 1; case 1: r = 2; break;")), Some("ATTR-7"));
    assert_eq!(rule(&sw("case 0: r = 1; [[fallthrough]]; case 1: r = 2; break;")), None);
    assert_eq!(rule(&sw("case 0: if (w) { r = 1; break; } case 1: r = 2; break;")), Some("ATTR-7"));
    assert_eq!(rule(&sw("case 0: case 1: r = 2; break;")), None);
    assert_eq!(rule(&sw("case 0: [[fallthrough]]; r = 1; case 1: break;")), Some("ATTR-2"));
    assert_eq!(rule(&sw("case 0: for (int i = 0; i < 2; ++i) { [[fallthrough]]; } case 1: break;")), Some("ATTR-2"));
    assert_eq!(rule("void f() { [[fallthrough]]; }"), Some("ATTR-2"));
    assert_eq!(rule("[[fallthrough]] int x;"), Some("ATTR-2"));
}

#[test]
fn attribute_placement_and_warnings() {
    let rule = |src: &str, flags: &[&str]| {
        let o = crate::Options { werror: true, wflags: flags.iter().map(|f| f.to_string()).collect(), ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    assert_eq!(rule("[[assume(true)]] int x;", &[]), Some("ATTR-13"));
    assert_eq!(rule("int [[maybe_unused]] c;", &[]), Some("ATTR-2"));
    assert_eq!(rule("using S [[nodiscard]] = int;", &[]), Some("ATTR-2"));
    assert_eq!(rule("using S [[deprecated]] = int;", &[]), None);
    assert_eq!(rule("void s(); [[noreturn]] void s();", &[]), Some("ATTR-2"));
    assert_eq!(rule("[[noreturn]] void s(); [[noreturn]] void s();", &[]), None);
    assert_eq!(rule("[[noreturn]] void s(int c) { (void)c; }", &[]), Some("ATTR-14"));
    assert_eq!(rule("void o(); [[deprecated]] void o(); void u() { o(); }", &[]), Some("ATTR-4"));
    assert_eq!(rule("void f() { int local = 1; }", &["-Wunused"]), Some("ATTR-6"));
    assert_eq!(rule("void f() { [[maybe_unused]] int local = 1; }", &["-Wunused"]), None);
    assert_eq!(rule("[[using gnu: gnu::unused]] int x;", &[]), Some("ATTR-17"));
}
