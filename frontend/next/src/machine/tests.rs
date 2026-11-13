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
}

#[test]
fn placeholder_return_types() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("auto& f() { static int x; return x; } int& r = f();"), None);
    assert_eq!(rule("auto& f() { int x = 1; return x; }"), Some("RET-3"));
    assert_eq!(rule("const auto& f() { int x = 1; return x; }"), None);
    assert_eq!(rule("int g; auto* f(bool c) { if (c) return &g; return nullptr; }"), Some("AUTO-19"));
    assert_eq!(rule("int g; auto* f() { return &g; } int* p = f();"), None);
    assert_eq!(rule("int g; decltype(auto) f() { return (g); } int& r = f();"), None);
    assert_eq!(rule("decltype(auto) f() { int x = 1; return (x); } int&& r = f();"), None);
    assert_eq!(rule("decltype(auto) f() { int x = 1; return (x); } int& r = f();"), Some("REF-9"));
    assert_eq!(rule("int g; auto&& f() { return g; } int& r = f();"), None);
    assert_eq!(rule("constexpr decltype(auto) f() { int x = 1; return (x); } static_assert(f() == 1);"), Some("SA-7"));
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

#[test]
fn specifier_agreement_and_c_names() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("consteval int one() { return 1; } int (*p)() = one;"), Some("CEV-2"));
    assert_eq!(rule("consteval int one() { return 1; } consteval int two() { return one(); }"), None);
    assert_eq!(rule("constexpr int f(); int f() { return 1; }"), Some("CX-5"));
    assert_eq!(rule("constexpr int f(); constexpr int f() { return 1; }"), None);
    assert_eq!(rule("extern constinit int y; int y = 1;"), Some("CINIT-2"));
    assert_eq!(rule("extern constinit int y; constinit int y = 1;"), None);
    assert_eq!(rule("void f(inline int x);"), Some("INL-2"));
    assert_eq!(rule("inline typedef int I;"), Some("INL-2"));
    assert_eq!(rule("namespace n { extern \"C\" int c; } int c;"), Some("LSP-7"));
    assert_eq!(rule("int c; namespace n { extern \"C\" int c; }"), Some("LSP-7"));
    assert_eq!(rule("int g(int); extern \"C\" int g(int);"), Some("LSP-7"));
    assert_eq!(rule("extern \"Fortran\" void f();"), Some("LSP-5"));
    assert_eq!(rule("extern \"Pascal\" { int g(); }"), Some("LSP-5"));
    assert_eq!(rule("extern \"C\" int g(int); int g(int);"), None);
}

#[test]
fn overload_sets_need_a_target() {
    let rule = |src: &str| match crate::check_source(&format!("void f(int); void f(double); {src}"), Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("void u() { f; }"), Some("OVL-20"));
    assert_eq!(rule("void u() { (void)f; }"), Some("OVL-20"));
    assert_eq!(rule("auto p = &f;"), Some("AUTO-8"));
    assert_eq!(rule("decltype(f) x;"), Some("DTY-4"));
    assert_eq!(rule("void g(...); void u() { g(f); }"), Some("OVL-20"));
    assert_eq!(rule("void g(void (&)(int)); void g(void (*)(int)); void u() { g(f); }"), Some("OVL-36"));
    assert_eq!(rule("void (*q)(int) = &f;"), None);
    assert_eq!(rule("void (&r)(double){f};"), None);
    assert_eq!(rule("void g(void (*)(int)); void u() { g(&f); }"), None);
    assert_eq!(rule("void (*q)(int) = (void (*)(int))f;"), None);
}

#[test]
fn small_declaration_checks() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("alignas(16) void f();"), Some("ALN-2"));
    assert_eq!(rule("alignas(16) typedef int I;"), Some("ALN-2"));
    assert_eq!(rule("alignas(16) int x;"), None);
    assert_eq!(rule("int main = 0;"), Some("TR-3"));
    assert_eq!(rule("namespace n { int main = 0; }"), None);
    assert_eq!(rule("void f(int a, int a);"), Some("FN-13"));
    assert_eq!(rule("void g(); void (*p)() = const_cast<void (*)()>(&g);"), Some("CAST-4"));
    assert_eq!(rule("void f(const int i) { i++; }"), Some("CV-4"));
    assert_eq!(rule("static int h(); int main() { return h(); }"), Some("ODR-9"));
    assert_eq!(rule("static int h(); int main() { return sizeof(h()) == 4 ? 0 : 1; }"), None);
}

#[test]
fn jumps_into_statements() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("void f() { goto in; if constexpr (true) { in:; } }"), Some("IF-17"));
    assert_eq!(rule("void f(int k) { switch (k) { case 0: if constexpr (true) { case 1:; } } }"), Some("IF-17"));
    assert_eq!(rule("void f() { if constexpr (true) { goto in; in:; } }"), None);
    assert_eq!(rule("void f(int k) { if constexpr (true) { switch (k) { case 1:; } } }"), None);
    assert_eq!(rule("int a[2]; int f() { goto in; for (int x : a) { in: return x; } return 0; }"), Some("JMP-6"));
    assert_eq!(rule("int a[2]; int f() { for (int x : a) { goto in; in: return x; } return 0; }"), None);
    assert_eq!(rule("int g(); int f() { goto in; while (int v = g()) { in: return v; } return 0; }"), Some("JMP-6"));
    assert_eq!(rule("int g(); int f() { while (int v = g()) { goto in; in: return v; } return 0; }"), None);
    assert_eq!(rule("int& keep(int v) { return v; }"), Some("REF-9"));
    assert_eq!(rule("int& keep(int& v) { return v; }"), None);
    assert_eq!(rule("int&& keep(int v) { return v; }"), None);
}

#[test]
fn literal_and_unevaluated_warnings() {
    let rule = |src: &str| {
        let o = crate::Options { werror: true, ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    };
    assert_eq!(rule("int c = 'ab';"), Some("PRIM-13"));
    assert_eq!(rule("int c = 'a';"), None);
    assert_eq!(rule("float x = 1e999f;"), Some("PRIM-13"));
    assert_eq!(rule("float x = 1e39f;"), Some("PRIM-13"));
    assert_eq!(rule("double x = 1e300;"), None);
    assert_eq!(rule("unsigned long f(int a[10]) { return sizeof(a); }"), Some("UN-19"));
    assert_eq!(rule("unsigned long f(int* a) { return sizeof(a); }"), None);
    assert_eq!(rule("int i = 0; auto n = sizeof(i++);"), Some("UNEV-4"));
    assert_eq!(rule("int i = 0; decltype(i = 1) j = i;"), Some("UNEV-4"));
    assert_eq!(rule("int i = 0; auto n = sizeof(i + 1);"), None);
}

#[test]
fn declaration_types() {
    let rule = |src: &str, m32: bool| match crate::check_source(src, Path::new("t.cpp"), &crate::Options { m32, ..Default::default() }).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("int a[3] = {1, 2, 3}; decltype(auto) b = a;", false), Some("AUTO-15"));
    assert_eq!(rule("int a[3] = {1, 2, 3}; decltype(auto) b = (a);", false), None);
    assert_eq!(rule("void g(); decltype(auto) f = g;", false), Some("AUTO-11"));
    assert_eq!(rule("int i = 0; decltype(auto) j = i;", false), None);
    assert_eq!(rule("char big[1ull << 32];", true), Some("DCL-19"));
    assert_eq!(rule("char big[1ull << 32];", false), None);
    assert_eq!(rule("int big[1ull << 31];", true), Some("DCL-19"));
    assert_eq!(rule("char t[(decltype(sizeof 0))-1][2];", false), Some("DCL-19"));
    assert_eq!(rule("void q() noexcept {} void (&c)(){q};", false), Some("REF-11"));
    assert_eq!(rule("void q() {} void (&c)(){q};", false), None);
}

#[test]
fn constants_inline_and_unread() {
    let rule = |src: &str, w: &[&str]| {
        let o = crate::Options { werror: !w.is_empty(), wflags: w.iter().map(|s| s.to_string()).collect(), ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
            _ => None,
        }
    };
    assert_eq!(rule("constexpr int&& c = 4; static_assert(c == 4);", &[]), Some("CE-3"));
    assert_eq!(rule("constexpr const int& c = 4; static_assert(c == 4);", &[]), None);
    assert_eq!(rule("constexpr int f() { int x = 7; void* v = &x; return *static_cast<int*>(v); } static_assert(f() == 7);", &[]), Some("CE-4"));
    assert_eq!(rule("constexpr int f() { int x = 7; void* v = &x; return v ? 1 : 0; } static_assert(f() == 1);", &[]), None);
    assert_eq!(rule("extern inline int s; int main() { return s; }", &[]), Some("INL-6"));
    assert_eq!(rule("extern inline int s; int main() { return sizeof(s) == 4 ? 0 : 1; }", &[]), None);
    assert_eq!(rule("inline int g(); int main() { return g(); }", &[]), None);
    assert_eq!(rule("void f() { int n; n = 3; }", &["-Wunused"]), Some("ATTR-6"));
    assert_eq!(rule("int f() { int n; n = 3; return n; }", &["-Wunused"]), None);
    assert_eq!(rule("void f() { int n = 0; n = n + 1; }", &["-Wunused"]), None);
}

#[test]
fn if_consteval() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    let w = "constexpr int w() { if consteval { return 1; } else { return 2; } } ";
    assert_eq!(rule(&format!("{}static_assert(w() == 1);", w)), None);
    assert_eq!(rule(&format!("{}static_assert(w() == 2);", w)), Some("SA-1"));
    let t = "consteval int t(int v) { return v * 2; } ";
    assert_eq!(rule(&format!("{}constexpr int f(int v) {{ if consteval {{ return t(v); }} return v; }}", t)), None);
    assert_eq!(rule(&format!("{}constexpr int f(int v) {{ if !consteval {{ return t(v); }} return v; }}", t)), Some("IF-24"));
    assert_eq!(rule(&format!("{}constexpr int f(int v) {{ return t(v); }}", t)), Some("CEV-1"));
    assert_eq!(rule("void f() { goto in; if consteval { in:; } }"), Some("IF-17"));
    assert_eq!(run(&format!("{}int c = w(); int o = 0; int d = w() + o; int main() {{ return c * 10 + d; }}", w)), RunOutcome::Result(12));
    assert_eq!(run(&format!("{}int main() {{ const int k = w(); int n = w(); return k * 10 + n; }}", w)), RunOutcome::Result(12));
}

#[test]
fn noexcept_specifiers_and_operator() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("void f() noexcept(2);"), Some("EXS-7"));
    assert_eq!(rule("constexpr int x = 0; void f() noexcept(&x);"), Some("EXS-7"));
    assert_eq!(rule("bool b = true; void f() noexcept(b);"), Some("EXS-7"));
    assert_eq!(rule("void f() noexcept(1); static_assert(noexcept(f()));"), None);
    assert_eq!(rule("void f() noexcept(nullptr); static_assert(!noexcept(f()));"), None);
    assert_eq!(rule("void f(int x) noexcept(noexcept(x + 1)); static_assert(noexcept(f(1)));"), None);
    assert_eq!(rule("void f(); static_assert(!noexcept(f()));"), None);
    assert_eq!(rule("void (*p)() noexcept = nullptr; static_assert(noexcept(p()));"), None);
    assert_eq!(rule("constexpr int f() { return 1; } static_assert(!noexcept(f()));"), None);
    assert_eq!(run("int c = 0; int g() { return ++c; } int main() { bool b = noexcept(g()); return c * 10 + b; }"), RunOutcome::Result(0));
}

#[test]
fn assumptions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        _ => None,
    };
    assert_eq!(rule("void f(int a, int b) { [[assume(a, b)]]; }"), Some("ATTR-13"));
    assert_eq!(rule("void f(int a, int b) { [[assume((a, b))]]; }"), None);
    assert_eq!(rule("constexpr int f(int v) { [[assume(v > 0)]]; return v; } static_assert(f(-1) == -1);"), Some("ATTR-10"));
    assert_eq!(rule("constexpr int f(int v) { [[assume(v > 0)]]; return v; } static_assert(f(1) == 1);"), None);
    assert_eq!(run("int main() { int i = 1; [[assume(++i > 0)]]; return i; }"), RunOutcome::Result(1));
    assert!(matches!(run("int main() { int i = 1; [[assume(i > 5)]]; return 0; }"), RunOutcome::Undefined("ATTR-10", ..)));
}

#[test]
fn braced_lists_for_arrays() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("const int f[2] = {1, 2}; int (&r)[2]{f};"), Some("REF-8"));
    assert_eq!(rule("int s[2]; int (&&r)[2]{s};"), Some("REF-10"));
    assert_eq!(rule("int s[2]; int (&r)[]{s};"), None);
    assert_eq!(rule("const int (&r)[2]{1, 2, 3};"), Some("REF-11"));
    assert_eq!(rule("int (&&r)[]{};"), Some("REF-11"));
    assert_eq!(rule("int (&r)[2]{1, 2};"), Some("REF-9"));
    assert_eq!(rule("constexpr const char (&r)[4]{\"ab\"}; static_assert(r[2] == 0 && r[3] == 0);"), None);
    assert_eq!(rule("using A = int[]; static_assert(sizeof(A{1, 2, 3}) == 3 * sizeof(int));"), None);
    assert_eq!(run("const int (&t)[3]{1, 2}; int (&&u)[]{3, 4}; int main() { u[0] += 1; return t[0] + t[2] * 10 + u[0] * 100 + u[1] * 1000; }"), RunOutcome::Result(4401));
    assert_eq!(run("using A = int[3]; int main() { return A{1, 2, 3}[1]; }"), RunOutcome::Result(2));
}

#[test]
fn structured_bindings_of_arrays() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(rule("int a[3]; auto [x, y] = a;"), Some("SB-2"));
    assert_eq!(rule("int a[2]; auto [x, x] = a;"), Some("SB-8"));
    assert_eq!(rule("void f() { int a[2]; auto [_, _] = a; }"), Some("SB-8"));
    assert_eq!(rule("int a[2]; auto [x, y] = (x, a);"), Some("SB-8"));
    assert_eq!(rule("int a[2]; auto [x, y] = a; int x;"), Some("SB-8"));
    assert_eq!(rule("int i; auto [x] = i;"), Some("SB-13"));
    assert_eq!(rule("int a[2] = {1, 2}; auto [p, q](a); auto [r, s]{a}; static auto [t, u] = a;"), None);
    assert_eq!(rule("int a[2]; auto& [x, y] = a; decltype(x) i = 0; int* p = &i;"), None);
    assert_eq!(run("int main() { int a[2] = {1, 2}; auto [x, y] = a; x = 10; a[1] = 5; return a[0] * 100 + x + y; }"), RunOutcome::Result(112));
    assert_eq!(run("int main() { int a[2] = {1, 2}; auto& [x, y] = a; x = 10; return a[0] + (&y == &a[1]); }"), RunOutcome::Result(11));
    assert_eq!(run("int a[2] = {3, 4}; auto [x, y] = a; int main() { return x * 10 + y; }"), RunOutcome::Result(34));
}

#[test]
fn pseudo_destructor_calls() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("using I = int; void f(int i, int* p) { i.~I(); p->~I(); p->I::~I(); }"), None);
    assert_eq!(rule("using L = long; void f(int i) { i.~L(); }"), Some("DTOR-8"));
    assert_eq!(rule("using I = int; void f(int i) { i.~I(1); }"), Some("DTOR-8"));
    assert_eq!(rule("int x; void f() { x.~decltype(x)(); }"), None);
    assert_eq!(rule("using I = int; constexpr int f() { int x = 1; x.~I(); x = 3; return 1; } static_assert(f() == 1);"), Some("OBJ-2"));
    assert_eq!(rule("using I = int; constexpr int f() { int x = 1; x.~I(); return 1; } static_assert(f() == 1);"), None);
    assert!(matches!(run("using I = int; int main() { int x = 1; x.~I(); return x; }"), RunOutcome::Undefined(..)));
}

#[test]
fn reinterpret_references_and_decay_copies() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("int main() { int x = 5; unsigned& u = reinterpret_cast<unsigned&>(x); u = 7; return x; }"), RunOutcome::Result(7));
    assert_eq!(run("int main() { int x = -1; return reinterpret_cast<unsigned&>(x) == 4294967295u; }"), RunOutcome::Result(1));
    assert_eq!(rule("const int c = 1; int& r = reinterpret_cast<int&>(c);"), Some("CAST-7"));
    assert_eq!(rule("constexpr int f() { int x = 1; return reinterpret_cast<unsigned&>(x); } static_assert(f() == 1);"), Some("CE-4"));
    assert_eq!(run("int main() { int v = 3; auto c = auto(v); c = 9; int a[2] = {1, 2}; auto p = auto{a}; return v * 10 + (p == a); }"), RunOutcome::Result(31));
    assert_eq!(rule("const int c = 2; auto a = auto(c); int* p = &a;"), None);
    assert_eq!(rule("auto b = auto{1, 2};"), Some("AUTO-20"));
    assert_eq!(rule("auto b = auto{{1}};"), Some("AUTO-20"));
}

#[test]
fn literal_suffixes_without_operators() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("auto x = 1lul;"), Some("PRIM-16"));
    assert_eq!(rule("int x = 1u7;"), Some("PRIM-16"));
    assert_eq!(rule("auto s = \"a\"_q;"), Some("PRIM-16"));
    assert_eq!(rule("int operator\"\"_k(unsigned long long); int x = 1_k;"), Some("unsupported"));
}

#[test]
fn references_in_constant_evaluation() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // AUTO-4: `auto&&` from an lvalue collapses to an lvalue reference.
    assert_eq!(rule("int t = 1; auto&& v = t; int& w = v; int a[2]; auto&& z = a; int q = z[0];"), None);
    assert_eq!(run("int main() { int t = 1; auto&& v = t; v = 4; return t; }"), RunOutcome::Result(4));
    assert_eq!(rule("static constexpr int t[] = {1, 2}; auto&& v = t; constexpr int s = v[1]; static_assert(s == 2);"), None);
    // CE-14: a reference to a local is not a constant initializer.
    assert_eq!(rule("int f() { constexpr int k = 3; const int& r = k; constexpr int s = r; return s; }"), Some("CE-3"));
    assert_eq!(rule("int f() { constexpr int k = 3; constexpr int s = k; return s; }"), None);
    assert_eq!(rule("void p(int) {} void (&c)(int) = p; void (&&d)(int){p}; static_assert(&c == &p && &d == &p);"), None);
}

#[test]
fn library_types_the_language_needs() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("auto a = {1, 2};"), Some("LSUP-1"));
    assert_eq!(rule("int main() { int s = 0; for (int v : {1, 2}) s += v; return s; }"), Some("LSUP-1"));
    assert_eq!(rule("auto r = 1 <=> 2;"), Some("LSUP-2"));
    assert_eq!(rule("namespace std { class strong_ordering; } auto r = 1 <=> 2;"), Some("unsupported"));
}

#[test]
fn default_new_alignment() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("static_assert(__STDCPP_DEFAULT_NEW_ALIGNMENT__ == 16);"), None);
    assert_eq!(rule("static_assert(sizeof(__STDCPP_DEFAULT_NEW_ALIGNMENT__) == sizeof(sizeof(0)));"), None);
    assert_eq!(rule("static_assert(__STDCPP_DEFAULT_NEW_ALIGNMENT__ == 8);"), Some("SA-1"));
}

#[test]
fn long_double_values() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("static_assert(9007199254740992.0L + 1.0L == 9007199254740993.0L);"), None);
    assert_eq!(rule("static_assert(9007199254740992.0 + 1.0 == 9007199254740992.0);"), None);
    assert_eq!(rule("constexpr long double x = 1.0L / 3; static_assert(x * 3 == 1.0L && (double)x == 1.0 / 3);"), None);
    assert_eq!(rule("constexpr long long n = 9007199254740993LL; static_assert((long long)(long double)n == n);"), None);
    assert_eq!(rule("static_assert(-2.5L < -2.0L && (int)-2.5L == -2 && 0.0L == -0.0L);"), None);
    assert_eq!(rule("long double x{9007199254740993LL}; double y{1.0L};"), None);
    assert_eq!(rule("double y{1e4000L};"), Some("LIST-2"));
    assert_eq!(rule("constexpr long double z = 1.0L / 0.0L;"), Some("CE-28"));
    assert_eq!(run("int main() { long double a = 0.1L; long double s = 0; for (int i = 0; i < 10; ++i) s += a; return s > 0.99L && s < 1.01L; }"), RunOutcome::Result(1));
    assert!(matches!(run("int main() { long double b = 0x1p16000L; int x = (int)b; return x; }"), RunOutcome::Undefined(..)));
}

#[test]
fn rules_for_forms_the_grammar_does_not_derive() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("void f() { register int x = 1; (void)x; }"), Some("STO-5"));
    assert_eq!(rule("int f(register int x) { return x; }"), Some("STO-5"));
    assert_eq!(rule("constexpr int f() { if consteval return 1; return 2; }"), Some("IF-20"));
    assert_eq!(rule("constexpr int f() { if consteval { return 1; } else return 2; }"), Some("IF-20"));
    assert_eq!(rule("void f() { if !constexpr (true) {} }"), Some("IF-20"));
    assert_eq!(rule("void f(int v) { if (static_assert(true); v) {} }"), Some("IF-21"));
    assert_eq!(rule("namespace n { int x; } void f(int v) { if (using namespace n; v) {} }"), Some("IF-21"));
    assert_eq!(rule("namespace n { int x; } void f(int v) { if (using I = int; v) {} }"), None);
    assert_eq!(rule("void f() { if (int x) {} }"), Some("IF-6"));
    assert_eq!(rule("void f() { if (int a = 1, b = 2) {} }"), Some("IF-6"));
    assert_eq!(rule("void f() { if (int a = 1; a) {} }"), None);
    assert_eq!(rule("void f() { do {} while (int x = 0); }"), Some("LOOP-6"));
    assert_eq!(rule("void f() { for (static_assert(true); ;) break; }"), Some("LOOP-7"));
}

#[test]
fn rules_for_declarations_the_grammar_does_not_derive() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("int a[2]; int [x, y] = a;"), Some("SB-15"));
    assert_eq!(rule("int a[2]; auto* [x, y] = a;"), Some("SB-15"));
    assert_eq!(rule("auto [x, y];"), Some("SB-15"));
    assert_eq!(rule("void f(auto [x, y]);"), Some("SB-15"));
    assert_eq!(rule("int a[2]; auto [x, y] = a;"), None);
    assert_eq!(rule("int a[2]; int g() { return a[0]; }"), None);
    assert_eq!(rule("void f() { namespace N {} }"), Some("NS-10"));
    assert_eq!(rule("namespace M {} void f() { namespace N = M; }"), None);
    assert_eq!(rule("inline namespace A::B {}"), Some("NS-11"));
    assert_eq!(rule("int x; using x;"), Some("NS-16"));
    assert_eq!(rule("using I alignas(16) = int;"), Some("ALN-2"));
    assert_eq!(rule("void f(alignas(16) int x);"), Some("ALN-2"));
    assert_eq!(rule("enum alignas(8) E { a };"), Some("ALN-7"));
    assert_eq!(rule("auto int a = 1;"), Some("AUTO-8"));
    assert_eq!(rule("void outer() { void inner() {} }"), Some("FN-9"));
    assert_eq!(rule("void outer() { void inner(); int x{}; (void)x; }"), None);
    assert_eq!(rule("void f() { extern \"C\" void g(); }"), Some("LSP-2"));
    assert_eq!(rule("constexpr const char* m = \"x\"; static_assert(true, m);"), Some("SA-8"));
    assert_eq!(rule("int f(int v) { switch (v) { case 1, 2: return 1; } return 0; }"), Some("COMMA-8"));
    assert_eq!(rule("typedef int;"), Some("TYN-2"));
}

#[test]
fn first_errors_name_the_specific_rule() {
    let named = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => (ds[0].rule, ds[0].cause),
        _ => (None, None),
    };
    assert_eq!(named("inline int main() { return 0; }"), (Some("TR-3"), Some("FN-4")));
    assert_eq!(named("int main() { return main(); }"), (Some("FN-4"), Some("TR-3")));
    assert_eq!(named("const int x = 1; int* p = &x;"), (Some("CV-10"), Some("CONV-9")));
    assert_eq!(named("void* v = nullptr; int* p = v;"), (Some("INIT-7"), Some("CONV-9")));
    assert_eq!(named("int* p; long* q = p;"), (Some("INIT-7"), None));
    assert_eq!(named("int&& r = 1; int&& s = r;"), (Some("REF-10"), Some("VC-14")));
    assert_eq!(named("int f(int a = 1); int (*p)(int) = f; int x = p();"), (Some("PFX-5"), Some("DEFARG-5")));
    assert_eq!(named("typedef int span; int span = 1;"), (Some("NAM-3"), Some("TYN-2")));
    assert_eq!(named("int X; typedef int X;"), (Some("NAM-3"), Some("TYN-2")));
    assert_eq!(named("int main() { { a: ; } { a: ; } }"), (Some("JMP-14"), Some("NAM-11")));
    assert_eq!(named("void f() noexcept; void f() noexcept(false) {}"), (Some("DCL-14"), Some("EXS-8")));
    assert_eq!(named("void f() { int l = 1; constexpr int* p = &l; }"), (Some("CX-2"), Some("CE-3")));
    assert_eq!(named("int val(); void f() { val() = 1; }"), (Some("ASN-2"), Some("VC-12")));
}

#[test]
fn pointer_reference_binding_and_indeterminate_bytes() {
    let src = "int x = 0; int* p = &x; int* make() { return &x; }\nint f(const int* const&) { return 1; } int f(const int*&&) { return 2; }\n";
    assert_eq!(run(&format!("{}int main() {{ const int* const& r = p; return &r == &p; }}", src)), RunOutcome::Result(1));
    assert_eq!(run(&format!("{}int main() {{ return f(p); }}", src)), RunOutcome::Result(1));
    assert_eq!(run(&format!("{}int main() {{ return f(make()); }}", src)), RunOutcome::Result(2));
    assert_eq!(run("int main() { unsigned char a; unsigned char b = a; (void)b; return 0; }"), RunOutcome::Result(0));
    assert_eq!(run("int main() { unsigned char a; unsigned char b; b = true ? a : a; (void)b; return 3; }"), RunOutcome::Result(3));
    assert!(matches!(run("int main() { unsigned char a; unsigned char b = a; return b; }"), RunOutcome::Undefined(..)));
    assert!(matches!(run("int main() { unsigned char a; int b = a; return b; }"), RunOutcome::Undefined(..)));
    assert_eq!(run("int main() { int a; (void)a; return 4; }"), RunOutcome::Result(4));
}

#[test]
fn calls_through_indirection_and_other_declarations() {
    assert_eq!(run("int twice(int v) { return v * 2; } int main() { int (*p)(int) = twice; return (*p)(3) + (**p)(1); }"), RunOutcome::Result(8));
    assert_eq!(run("int f(int a) { return a; } int g() { int f(int a = 7); return f(); } int main() { return g(); }"), RunOutcome::Result(7));
    assert_eq!(run("namespace { extern \"C\" int code() { return 3; } } namespace b { extern \"C\" int code(); } int main() { return b::code(); }"), RunOutcome::Result(3));
    assert!(matches!(run("int main() { int (*p)() = nullptr; return (*p)(); }"), RunOutcome::Undefined(..)));
}
