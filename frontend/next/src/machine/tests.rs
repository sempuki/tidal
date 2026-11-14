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
    assert!(matches!(outcome("template <class T> struct S { T a; };\nstatic_assert(1 == 2);"), Outcome::Unsupported(_)));
    assert!(matches!(outcome("static_assert(1 == 2);\ntemplate <class T> struct S { T a; };"), Outcome::Reject(_)));
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
    assert!(matches!(crate::check_source("const int c = 1; int& r = reinterpret_cast<int&>(c);", Path::new("t.cpp"), &crate::Options::default()).0, Outcome::Reject(ds) if ds[0].rule == Some("CAST-7")));
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
    assert_eq!(named("int f(int a, int b) { return a ?: b; }"), (Some("COND-14"), None));
    assert_eq!(named("int f() { return (1, {2}); }"), (Some("COMMA-9"), None));
    assert_eq!(named("int f() { if ({1}, true) return 1; return 0; }"), (Some("COMMA-9"), None));
    assert_eq!(named("int a[2]; auto ([x, y]) = a;"), (Some("SB-15"), None));
    assert_eq!(named("int a[2]; auto [x, y] = a, z = 1;"), (Some("SB-15"), None));
    assert_eq!(named("namespace n { int a = 1; } int main() { return a; }"), (Some("NAM-8"), Some("NAM-9")));
    assert_eq!(named("int main() { return b; }"), (Some("NAM-8"), None));
    assert_eq!(named("namespace A { int x; } namespace B { int x; } using namespace A; using namespace B; int y = x;"), (Some("NAM-26"), Some("NS-17")));
    assert_eq!(named("void f() { int x = 0; extern int x; }"), (Some("NAM-3"), Some("STO-13")));
    assert_eq!(named("static typedef int A;"), (Some("STO-6"), Some("TYN-2")));
    assert_eq!(named("void f() { if constexpr (false) { g(); } }"), (Some("NAM-8"), Some("IF-23")));
    assert_eq!(named("char* p = \"abc\";"), (Some("CV-10"), Some("PRIM-19")));
    assert_eq!(named("volatile int v; int* p = &v;"), (Some("CV-10"), Some("VOL-10")));
    assert_eq!(named("int x{.a = 1};"), (Some("LIST-11"), Some("LIST-14")));
    assert_eq!(named("const int x = 1; int& r = x;"), (Some("REF-8"), None));
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

/// Run a program of several units, written to files in a scratch
/// directory of the test.
fn run_units(name: &str, units: &[&str]) -> RunOutcome {
    let dir = std::env::temp_dir().join(format!("tidal-next-link-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let paths: Vec<std::path::PathBuf> = units.iter().enumerate().map(|(k, src)| {
        let p = dir.join(format!("u{}.cpp", k));
        std::fs::write(&p, src).unwrap();
        p
    }).collect();
    let (outs, u) = crate::check_program(&paths, &crate::Options::default());
    let _ = std::fs::remove_dir_all(&dir);
    assert!(outs.iter().all(|o| matches!(o, Outcome::Accept)), "a unit is not accepted");
    run_program(&u.unwrap())
}

#[test]
fn programs_of_several_units() {
    assert_eq!(run_units("fn", &["int f(); int main() { return f(); }", "int f() { return 7; }"]), RunOutcome::Result(7));
    assert_eq!(run_units("var", &["extern int x; int main() { return x; }", "int x = 5;"]), RunOutcome::Result(5));
    assert_eq!(run_units("ns", &["namespace n { int f(); } int main() { return n::f(); }", "namespace n { int f() { return 2; } }"]), RunOutcome::Result(2));
    // Internal linkage: each unit has its own x and f.
    assert_eq!(run_units("static", &["static int x = 1; static int f() { return 10; } int g(); int main() { return x + f() + g(); }", "static int x = 2; static int f() { return 20; } int g() { return x + f(); }"]), RunOutcome::Result(33));
    // INL-1: an inline variable and an inline function are one entity.
    assert_eq!(run_units("inline", &["inline int v = 5; inline int t(int a) { return a; } const int* p(); int (*q())(int); int main() { return (&v == p()) + 2 * (&t == q()); }", "inline int v = 5; inline int t(int a) { return a; } const int* p() { return &v; } int (*q())(int) { return &t; }"]), RunOutcome::Result(3));
    // LSP-8: a C variable and a global variable of another unit.
    assert_eq!(run_units("c", &["extern \"C\" int shared; int main() { return shared; }", "int shared = 4;"]), RunOutcome::Result(4));
    assert_eq!(run("static int x = 1; extern int x; int main() { return x; }"), RunOutcome::Result(1));
}

#[test]
fn the_byte_view_of_objects() {
    // STG-26: little-endian bytes, and a pointer's bytes are its address.
    assert_eq!(run("int main() { unsigned v = 0x04030201; const unsigned char* b = reinterpret_cast<const unsigned char*>(&v); return b[0] + 10 * b[3]; }"), RunOutcome::Result(41));
    assert_eq!(run("int main() { int n = -1; return reinterpret_cast<unsigned char*>(&n)[2]; }"), RunOutcome::Result(255));
    assert_eq!(run("int t; int main() { int* p = &t; auto a = reinterpret_cast<unsigned long>(p); return reinterpret_cast<int*>(a) == p; }"), RunOutcome::Result(1));
    // STG-13: a byte pointer walks the object and ends one past it.
    assert_eq!(run("int main() { long v = 0x0102030405060708; const unsigned char* p = reinterpret_cast<const unsigned char*>(&v); int s = 0; for (const unsigned char* e = p + sizeof v; p != e; ++p) s += *p; return s; }"), RunOutcome::Result(36));
    assert_eq!(run("int main() { int a[3] = {}; return static_cast<const void*>(a + 3) == static_cast<const void*>(reinterpret_cast<unsigned char*>(a) + sizeof a); }"), RunOutcome::Result(1));
    // A byte write changes the object.
    assert_eq!(run("int main() { int x = 0; reinterpret_cast<unsigned char*>(&x)[1] = 1; return x == 256; }"), RunOutcome::Result(1));
    // STG-23, STG-29, ADD-14.
    assert!(matches!(run("int main() { int v = 0; *reinterpret_cast<signed char*>(&v) = 1; return v; }"), RunOutcome::Undefined("STG-23", ..)));
    assert!(matches!(run("int main() { alignas(int) unsigned char b[4]; int* p = reinterpret_cast<int*>(b); *p = 5; return 0; }"), RunOutcome::Undefined("STG-23", ..)));
    assert!(matches!(run("alignas(int) unsigned char pool[16]; int main() { int* f = static_cast<int*>(static_cast<void*>(pool)); int* g = f + 2; return g == f; }"), RunOutcome::Undefined("ADD-14", ..)));
    assert_eq!(run("alignas(int) unsigned char pool[16]; int main() { int* f = static_cast<int*>(static_cast<void*>(pool)); unsigned char* n = static_cast<unsigned char*>(static_cast<void*>(f + 1)); return n - pool; }"), RunOutcome::Result(4));
    // ENUM-6 after a copy of bytes, and PFX-19.
    assert!(matches!(run("enum C { r, g }; int main() { C c = r; int s = 5; unsigned char* t = reinterpret_cast<unsigned char*>(&c); t[0] = reinterpret_cast<unsigned char*>(&s)[0]; C d = c; return d; }"), RunOutcome::Undefined("ENUM-6", ..)));
    assert!(matches!(run("void f(long) {} int main() { auto p = reinterpret_cast<void (*)(int)>(&f); p(1); return 0; }"), RunOutcome::Undefined("PFX-19", ..)));
}

#[test]
fn classes_with_data_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // Layout, aggregate initialization with brace elision, member access.
    assert_eq!(run("struct P { int x = 1; int y; }; P p{2, 3}; P q; int main() { P r = p; r.y += 10; P a[2] = {{1, 2}, 5, 6}; P* pp = &a[1]; return p.x * 1000 + r.y * 10 + q.x + pp->y * 10000; }"), RunOutcome::Result(62131));
    assert_eq!(rule("struct S { char c; int i; char d; }; static_assert(sizeof(S) == 12 && alignof(S) == 4);"), None);
    assert_eq!(rule("struct S { int a; }; struct T { S s; int b; }; constexpr T t{{1}, 2}; static_assert(t.s.a + t.b == 3);"), None);
    assert_eq!(rule("struct S { int a; }; S s{1, 2};"), Some("LIST-12"));
    assert_eq!(rule("struct S { int a; int b; }; S s{.b = 1, .a = 2};"), Some("LIST-14"));
    assert_eq!(rule("struct S { int a; }; int f(S s) { return s.b; }"), Some("PFX-11"));
    assert_eq!(rule("class C { int x; }; int f(C c) { return c.x; }"), Some("ACC-2"));
    assert_eq!(rule("struct S; struct S s;"), Some("INIT-7"));
    assert_eq!(rule("struct S { S s; };"), Some("CLS-1"));
    // Deleted special members and volatile copies.
    assert_eq!(rule("struct S { const int c; }; S s;"), Some("CTOR-20"));
    assert_eq!(rule("struct S { const int c = 1; }; void f(S& a, const S& b) { a = b; }"), Some("CPY-23"));
    assert_eq!(rule("struct S { int v; }; volatile S a{}; S b = a;"), Some("VOL-11"));
    // Constant evaluation of class values.
    assert_eq!(rule("struct P { int a; int b; }; constinit P p;"), Some("CINIT-1"));
    assert_eq!(rule("struct T { mutable int c; int f; }; constexpr T s{1, 2}; constexpr T t = s;"), Some("CX-1"));
    // A class and a variable of one name in one scope: the variable hides
    // the class.
    assert_eq!(rule("struct node { int v = 4; }; int node = 3; int main() { struct node n; return n.v + node; }"), None);
    // Anonymous unions wait for a later part.
    assert_eq!(rule("struct A { union { int a; }; };"), Some("unsupported"));
    // The bytes of a class, with padding.
    assert_eq!(run("struct R { char t; int c; }; int main() { R a{'a', 3}, b{}; auto* s = reinterpret_cast<unsigned char*>(&a); auto* d = reinterpret_cast<unsigned char*>(&b); for (unsigned i = 0; i < sizeof(R); ++i) d[i] = s[i]; return b.c; }"), RunOutcome::Result(3));
}

#[test]
fn member_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let counter = "class Counter { int n = 0; public: void add(int k) { n += k; } int get() const { return n; } int twice() const { return get() * 2; } static int unit() { return 1; } };\n";
    assert_eq!(run(&format!("{}int main() {{ Counter c; c.add(3); c.add(Counter::unit()); const Counter& r = c; return r.twice(); }}", counter)), RunOutcome::Result(8));
    assert_eq!(rule(&format!("{}int f(const Counter& c) {{ return c.add(1), 0; }}", counter)), Some("OVL-35"));
    assert_eq!(rule(&format!("{}int f(Counter c) {{ return c.n; }}", counter)), Some("ACC-2"));
    // Out-of-class definitions, `this`, overloads on the object's cv.
    assert_eq!(run("struct S { int x; int y() const; int f() { return 1; } int f() const { return 2; } };\nint S::y() const { return this->x; }\nint main() { S s{3}; const S& c = s; return s.y() * 100 + s.f() * 10 + c.f(); }"), RunOutcome::Result(312));
    assert_eq!(rule("struct S { void f(); void f(); };"), Some("CLS-9"));
    assert_eq!(rule("struct S { static void f() const; };"), Some("CLS-27"));
    assert_eq!(rule("struct S { void S(); };"), Some("CTOR-1"));
    assert_eq!(rule("struct C { }; C::C* p = nullptr;"), Some("NAM-34"));
    assert_eq!(rule("struct S { int v; }; int f() { return S::v; }"), Some("PRIM-25"));
    assert_eq!(rule("struct S { int v; }; static_assert(sizeof(S::v) == sizeof(int));"), None);
    // A static member function has no object, and the linkage of its
    // class.
    assert_eq!(rule("struct C { static int f(int); }; int (*p)(int) = C::f;"), None);
    assert_eq!(rule("struct S { static int n; };"), None);
}

#[test]
fn constructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // Member initializers, delegation, explicit, converting constructors.
    let p = "struct P { int a; int b; P() : a(1), b{2} {} P(int x) : P() { a = x; } explicit P(int x, int y) : a(x), b(y) {} int sum() const { return a + b; } };\n";
    assert_eq!(run(&format!("{}P g(5);\nint main() {{ P p; P q(3); P r{{4, 5}}; P s = 6; P t = P(7, 8); return p.sum() + q.sum() * 10 + r.sum() * 100 + s.sum() * 1000 + g.a * 10000 + t.sum(); }}", p)), RunOutcome::Result(58968));
    assert_eq!(rule(&format!("{}P p = {{1, 2}};", p)), Some("CTOR-9"));
    // The object is constructed in its place, so `this` is its address.
    assert_eq!(run("struct S { S* self; S() : self(this) {} }; int main() { S s; return s.self == &s; }"), RunOutcome::Result(1));
    // Checks.
    assert_eq!(rule("struct M { M(int); }; struct S { M m; }; S s;"), Some("CTOR-20"));
    assert_eq!(rule("struct S { S(int); }; S s;"), Some("CTOR-19"));
    assert_eq!(rule("struct S { S(int) : S('a') {} S(char) : S(1) {} };"), Some("CTOR-14"));
    assert_eq!(rule("struct S { int a; S() : b(1) {} };"), Some("CTOR-15"));
    assert_eq!(rule("struct S { S(); S(); };"), Some("CTOR-1"));
    assert_eq!(rule("struct S { S() {} S(S) {} };"), Some("CPY-15"));
    assert_eq!(rule("struct A { A(int) {} }; struct B { B(A) {} }; B b = 1;"), Some("CTOR-22"));
    assert_eq!(rule("void nothing(); struct S { S() { return nothing(); } };"), Some("JMP-12"));
    assert_eq!(rule("struct S { S(int) = delete; }; S s(1);"), Some("DEL-1"));
    // A constant initializer through a constexpr constructor.
    assert_eq!(rule("struct P { int x; constexpr P(int v) : x(v) {} }; constexpr P p(3); static_assert(p.x == 3);"), None);
    assert_eq!(rule("struct P { int f; int s; constexpr P() : f(1) {} }; constinit P p;"), Some("CINIT-1"));
}

#[test]
fn destructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let d = "int log_ = 0;\nstruct D { int id; D(int i) : id(i) {} ~D() { log_ = log_ * 10 + id; } };\n";
    // Block objects end in reverse order; an elided temporary is one object.
    assert_eq!(run(&format!("{}int main() {{ {{ D a(1); D b(2); }} {{ D t = D(3); }} return log_; }}", d)), RunOutcome::Result(213));
    // A temporary ends with its full expression; a bound one with the
    // reference.
    assert_eq!(run(&format!("{}int main() {{ D(4).id; log_ = log_ * 10 + 5; {{ const D& r = D(6); log_ = log_ * 10 + 7; }} return log_; }}", d)), RunOutcome::Result(4576));
    // Members are destroyed after the body, last declared first.
    assert_eq!(run("int log_ = 0; struct M { int v; ~M() { log_ = log_ * 10 + v; } }; struct H { M a{1}; M b{2}; ~H() { log_ = log_ * 10 + 9; } }; int main() { { H h; } return log_; }"), RunOutcome::Result(921));
    assert_eq!(rule("struct S { ~S() = delete; }; void f() { S s; }"), Some("DTOR-3"));
    assert_eq!(rule("struct S { ~S() = delete; }; S make(); void f() { make(); }"), Some("DTOR-3"));
    assert_eq!(rule("struct S { ~S() = delete; }; S make(); using T = decltype(make());"), None);
    assert_eq!(rule("struct S { int ~S(); };"), Some("DTOR-1"));
    assert_eq!(rule("struct S { int v; ~S() {} }; constexpr S s{1};"), Some("CX-1"));
}

#[test]
fn lambdas() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A capture by copy copies at the lambda expression; one by
    // reference sees later changes.
    assert_eq!(run("int main() { int n = 1; auto g = [n] { return n; }; auto h = [&n] { return n; }; n = 5; return g() * 10 + h(); }"), RunOutcome::Result(15));
    // Capture defaults, mutable state in a copy, and nested captures.
    assert_eq!(run("int main() { int a = 2, b = 3; auto f = [=, &b] { b = a + b; return b; }; f(); return b; }"), RunOutcome::Result(5));
    assert_eq!(run("int main() { auto c = [k = 0]() mutable { return ++k; }; c(); auto d = c; d(); return c() * 10 + d(); }"), RunOutcome::Result(23));
    assert_eq!(run("int main() { int a = 4; auto outer = [&] { return [=] { return a + 1; }(); }; return outer(); }"), RunOutcome::Result(5));
    // An array is copied element by element; a constant read needs no
    // capture.
    assert_eq!(run("int main() { int a[2] = {1, 2}; auto f = [a]() mutable { a[0] = 9; return a[0] + a[1]; }; return f() + a[0]; }"), RunOutcome::Result(12));
    assert_eq!(run("int main() { const int k = 7; constexpr int t[] = {1, 2}; return [] { return k + t[1]; }(); }"), RunOutcome::Result(9));
    // A lambda without captures converts to a pointer to function.
    assert_eq!(run("int main() { int (*p)(int) = [](int x) { return x + 1; }; auto q = +[](int x) { return x * 3; }; return p(1) + q(2); }"), RunOutcome::Result(8));
    assert_eq!(rule("constexpr auto sq = [](int v) { return v * v; }; static_assert(sq(4) == 16);"), None);
    assert_eq!(rule("void f() { int n = 0; int (*p)() = [n] { return n; }; }"), Some("INIT-7"));
    assert_eq!(rule("void f() { int a = 1; [] { return a; }; }"), Some("LAM-9"));
    assert_eq!(rule("void f() { int a = 1; [=, a] { return a; }; }"), Some("LAM-6"));
    assert_eq!(rule("void f() { int a = 1; [a, a] { return a; }; }"), Some("LAM-6"));
    assert_eq!(rule("auto g = [=] { return 1; };"), Some("LAM-6"));
    assert_eq!(rule("void f() { int n = 0; [n] { ++n; }; }"), Some("LAM-23"));
    assert_eq!(rule("void f() { const int c = 4; [] { const int* p = &c; return *p; }; }"), Some("LAM-17"));
    assert_eq!(rule("void f() { int x = 1; auto a = [x] { return x; }; auto b = a; b = a; }"), Some("LAM-26"));
    assert_eq!(rule("auto g = [](bool b) { if (b) return 1; return 2.0; };"), Some("LAM-22"));
    assert_eq!(rule("int f() { auto g = [=]() static { return 1; }; return g(); }"), Some("LAM-27"));
}

#[test]
fn static_data_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct S { static int count; S() { ++count; } }; int S::count = 1; int main() { S a, b; return S::count; }"), RunOutcome::Result(3));
    assert_eq!(run("struct S { static const int k = 3; static constexpr int c = 4; inline static int i = 5; int f() { return k + c + i; } }; int main() { S s; return s.f() + s.k; }"), RunOutcome::Result(15));
    assert_eq!(run("struct S { int v = 3; static S self; }; S S::self; int main() { return S::self.v; }"), RunOutcome::Result(3));
    assert_eq!(rule("struct S { static int n = 4; };"), Some("CLS-23"));
    assert_eq!(rule("int n = 1; struct S { static const int k = n; };"), Some("CLS-20"));
    assert_eq!(rule("void f() { struct S { static int n; }; }"), Some("CLS-28"));
    assert_eq!(rule("struct { static int n; } x;"), Some("CLS-9"));
    assert_eq!(rule("struct S { static auto n = 1; };"), Some("AUTO-17"));
    assert_eq!(rule("struct B { static int c; }; int B::c = 0; int B::c = 1;"), Some("ODR-1"));
    assert_eq!(rule("struct S { static int n; }; using S::n;"), Some("NS-9"));
    assert_eq!(rule("struct L { static constexpr int top = 9; }; constexpr int L::top; const int* p = &L::top;"), None);
    assert_eq!(rule("struct C { static constexpr int v = 1; }; static_assert(C::C::v == 1);"), None);
}

#[test]
fn copy_and_move_constructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let m = "int copies = 0; int moves = 0; struct M { M() {} M(const M&) { ++copies; } M(M&&) { ++moves; } };\n";
    // An lvalue is copied, an xvalue moved; a returned local is moved
    // and a prvalue initializes its object directly.
    assert_eq!(run(&format!("{}M make() {{ M x; return x; }} int main() {{ M a; M b(a); M c = static_cast<M&&>(a); M d = make(); M e = M(); return copies * 10 + moves; }}", m)), RunOutcome::Result(12));
    // The implicit copy copies each member; a user-declared destructor
    // removes the implicit move.
    assert_eq!(run(&format!("{}struct S {{ M m; int k = 1; }}; struct D {{ M m; ~D() {{}} }}; int main() {{ S a; S b = a; S c = static_cast<S&&>(a); D d; D e = static_cast<D&&>(d); return copies * 10 + moves; }}", m)), RunOutcome::Result(21));
    assert_eq!(run("struct C { int v; C(int x) : v(x) {} }; int main() { C a(4); C b(a); return b.v; }"), RunOutcome::Result(4));
    assert_eq!(rule("struct C { C() {} C(C&&) {} }; void f() { C a; C b = a; }"), Some("CPY-21"));
    assert_eq!(rule("struct C { C() {} C(C&) {} }; void f(const C& a) { C b = a; }"), Some("CPY-20"));
    assert_eq!(rule("struct C { C() {} C(const C&) = delete; }; void f() { C a; C b = a; }"), Some("DEL-1"));
    assert_eq!(rule("struct C { C() {} explicit C(const C&) {} }; void f() { C a; C b = a; }"), Some("CPY-24"));
    assert_eq!(rule("struct S { S() {} S(const volatile S&) = default; }; S a; S b(a);"), Some("CPY-26"));
    assert_eq!(rule("struct M { M() {} M(const M&) = delete; }; struct S { M m; S() {} S(const S&); }; S::S(const S&) = default;"), Some("CPY-17"));
}

#[test]
fn operator_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // Member and non-member operator functions, by operator and by name.
    assert_eq!(run("struct V { int x; V operator+(const V& o) const { return V{x + o.x}; } }; V operator-(V a, V b) { return V{a.x - b.x}; } int main() { V a{1}, b{2}; V c = a + b; c = a.operator+(b); return operator-(c, a).x + (c - b).x; }"), RunOutcome::Result(3));
    // Prefix and postfix increment, subscript, call, and != from ==.
    assert_eq!(run("struct C { int n = 0; C& operator++() { ++n; return *this; } C operator++(int) { C t = *this; ++n; return t; } int operator[](int i) const { return n * i; } int operator()(int a, int b) const { return a + b + n; } bool operator==(const C& o) const { return n == o.n; } }; int main() { C c; ++c; c++; C d; return c[3] * 10 + c(1, 2) + (c != d) * 100 + (d == c); }"), RunOutcome::Result(165));
    // Assignment operators; the implicit assignment assigns each member.
    assert_eq!(run("int k = 0; struct M { M& operator=(const M&) { ++k; return *this; } }; struct S { M m; int v; }; struct A { int v; A& operator=(int x) { v = x; return *this; } }; int main() { A a{1}; a = 5; S s{}, t{}; s = t; return a.v * 10 + k; }"), RunOutcome::Result(51));
    // The right operand of an overloaded assignment comes first.
    assert_eq!(run("int log_ = 0; struct V { V& operator=(int) { return *this; } }; V v; V& left() { log_ = log_ * 10 + 1; return v; } int right() { log_ = log_ * 10 + 2; return 1; } int main() { left() = right(); return log_; }"), RunOutcome::Result(21));
    // A converting constructor converts an operand of a non-member.
    assert_eq!(run("struct S { int v; S(int x) : v(x) {} }; S operator+(S a, S b) { return S(a.v + b.v); } int main() { S s = 1 + S(2); return s.v; }"), RunOutcome::Result(3));
    assert_eq!(rule("struct S {}; int operator+(int, int);"), Some("OPR-2"));
    assert_eq!(rule("struct S {}; S& operator=(S&, int);"), Some("OPR-2"));
    assert_eq!(rule("struct S { S operator++(double); };"), Some("OPR-2"));
    assert_eq!(rule("struct S { S& operator++(); }; void f(S s) { s++; }"), Some("OPR-13"));
    assert_eq!(rule("struct S {}; void f(S a, S b) { a * b; }"), Some("MUL-3"));
    assert_eq!(rule("struct S { int v; int operator==(int o) const { return v == o; } }; bool f(S s) { return s != 1; }"), Some("DCMP-12"));
    assert_eq!(rule("enum C { r, b }; bool operator==(const C&, const C&); bool x = r == b;"), Some("OVL-36"));
    assert_eq!(rule("struct S { void operator=(const S&) = default; };"), Some("CPY-17"));
}

#[test]
fn derived_classes() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A base is initialized first and destroyed last; its members are
    // members of the derived class.
    assert_eq!(run("int log_ = 0; struct B { int v; B(int x) : v(x) { log_ = log_ * 10 + 1; } ~B() { log_ = log_ * 10 + 3; } }; struct D : B { int w; D() : B(4), w(5) { log_ = log_ * 10 + 2; } ~D() { log_ = log_ * 10 + 4; } }; int main() { { D d; if (d.v + d.w != 9) return 1; } return log_ == 1243 ? 0 : 2; }"), RunOutcome::Result(0));
    // A derived object converts to a reference and a pointer to its base,
    // and the base's member functions take it as their object.
    assert_eq!(run("struct A { int a = 1; int get() const { return a; } }; struct B { int b = 2; }; struct D : A, B { int d = 3; }; int f(const B& b) { return b.b; } int main() { D x; B* p = &x; return x.get() * 100 + f(x) * 10 + p->b + x.d; }"), RunOutcome::Result(125));
    // An empty base takes no space.
    assert_eq!(run("struct E {}; struct D : E { int v; }; int main() { return sizeof(D) == sizeof(int) ? 0 : 1; }"), RunOutcome::Result(0));
    assert_eq!(rule("struct B {}; struct D : B, B {};"), Some("DER-1"));
    assert_eq!(rule("struct A { int x; }; struct B { int x; }; struct D : A, B { int f() { return x; } };"), Some("DER-4"));
    assert_eq!(rule("struct B { int x; }; struct D : private B {}; int f(D d) { return d.x; }"), Some("ACC-2"));
    assert_eq!(rule("struct A {}; struct B : private A {}; struct C : B { void f() { A* p = nullptr; (void)p; } };"), Some("ACC-14"));
    assert_eq!(rule("struct B {}; struct D : virtual B {};"), Some("unsupported"));
}

#[test]
fn virtual_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A call through a reference or pointer reaches the final overrider;
    // a qualified call does not.
    assert_eq!(run("struct B { virtual int f() const { return 1; } }; struct M : B {}; struct D : M { int f() const override { return 2; } int base() const { return B::f(); } }; int g(const B& b) { return b.f(); } int main() { D d; B b; const B* p = &d; return g(d) * 100 + g(b) * 10 + d.base() + p->f(); }"), RunOutcome::Result(213));
    // During the constructor and destructor of a base, the object is of
    // the base's class.
    assert_eq!(run("int log_ = 0; struct B { B() { log_ = log_ * 10 + id(); } virtual ~B() { log_ = log_ * 10 + id(); } virtual int id() const { return 1; } }; struct D : B { D() { log_ = log_ * 10 + id(); } ~D() { log_ = log_ * 10 + id(); } int id() const override { return 2; } }; int main() { { D d; } return log_ == 1221 ? 0 : 1; }"), RunOutcome::Result(0));
    // A covariant result converts to the base's type; an override in a
    // second base overrides both.
    assert_eq!(run("struct L { int l = 1; }; struct R { int r = 2; virtual R* self() { return this; } }; struct D : L, R { int d = 3; D* self() override { return this; } }; int main() { D d; R& r = d; return r.self()->r; }"), RunOutcome::Result(2));
    // A polymorphic class holds a pointer before its members.
    assert_eq!(run("struct S { virtual void f() {} int v; }; int main() { return sizeof(S) == 2 * sizeof(void*) ? 0 : 1; }"), RunOutcome::Result(0));
    let r = run("struct B { B() { f(); } virtual void f() = 0; }; void B::f() {} struct D : B { void f() override {} }; int main() { D d; return 0; }");
    assert!(matches!(r, RunOutcome::Undefined("VIRT-12", ..)), "{:?}", r);
    assert_eq!(rule("struct A { virtual void f() = 0; }; A a;"), Some("VIRT-8"));
    assert_eq!(rule("struct A { virtual void f() = 0; }; struct B : A {}; void g() { B b; }"), Some("VIRT-8"));
    assert_eq!(rule("struct B { virtual void f(); }; struct D : B { void f(int) override; };"), Some("VIRT-3"));
    assert_eq!(rule("struct B { virtual void f() final; }; struct D : B { void f(); };"), Some("VIRT-3"));
    assert_eq!(rule("struct B { virtual int f(); }; struct D : B { long f(); };"), Some("VIRT-15"));
    assert_eq!(rule("struct B { virtual void f(); }; struct D : B { static void f(); };"), Some("VIRT-21"));
    assert_eq!(rule("struct S { void f() = 0; };"), Some("VIRT-9"));
    assert_eq!(rule("virtual void f();"), Some("VIRT-2"));
    assert_eq!(rule("struct B { virtual void f(); }; struct D : virtual B {};"), Some("unsupported"));
}

#[test]
fn new_and_delete() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // An object and an array with initializers; delete runs the
    // destructors, the elements from the last to the first.
    assert_eq!(run("int main() { int* p = new int(4); auto* q = new auto(3); int* a = new int[3]{1, 2}; int r = *p * 100 + *q * 10 + a[0] + a[1] + a[2]; delete p; delete q; delete[] a; return r; }"), RunOutcome::Result(433));
    assert_eq!(run("int log_ = 0; struct S { int k; S(int k) : k(k) {} S() : k(9) {} ~S() { log_ = log_ * 10 + k; } }; int main() { int n = 2; S* a = new S[n + 1]{1, 2}; delete[] a; S* s = new S(5); delete s; return log_; }"), RunOutcome::Result(9215));
    // delete through a base with a virtual destructor runs the derived one.
    assert_eq!(run("int log_ = 0; struct B { virtual ~B() { log_ = log_ * 10 + 1; } }; struct D : B { ~D() { log_ = log_ * 10 + 2; } }; int main() { B* p = new D; delete p; return log_; }"), RunOutcome::Result(21));
    let r = run("int main() { int* p = new int; int v = *p; delete p; return v; }");
    assert!(matches!(r, RunOutcome::Undefined("NEW-3", ..)), "{:?}", r);
    let r = run("int main() { int* p = new int[2]; delete p; return 0; }");
    assert!(matches!(r, RunOutcome::Undefined("NEW-5", ..)), "{:?}", r);
    let r = run("struct B { ~B() {} }; struct D : B {}; int main() { B* p = new D; delete p; return 0; }");
    assert!(matches!(r, RunOutcome::Undefined("VIRT-12", ..)), "{:?}", r);
    // Constant evaluation frees what it allocates.
    assert_eq!(rule("constexpr int f() { int* p = new int(1); return *p; } static_assert(f() == 1);"), Some("SA-7"));
    assert_eq!(rule("constexpr int f() { int* p = new int(1); int v = *p; delete p; return v; } static_assert(f() == 1);"), None);
    assert_eq!(rule("constexpr bool f() { int* p = new int; delete p; delete p; return true; } static_assert(f());"), Some("SA-7"));
    assert_eq!(rule("struct A { virtual void f() = 0; }; void g() { new A; }"), Some("NEW-7"));
    assert_eq!(rule("void g(void* p) { delete p; }"), Some("NEW-8"));
    assert_eq!(rule("void g() { new auto(1, 2); }"), Some("NEW-16"));
    assert_eq!(rule("void g() { new int[2]{1, 2, 3}; }"), Some("NEW-17"));
    assert_eq!(rule("void g(int n) { new int[2][n]; }"), Some("NEW-11"));
    assert_eq!(rule("void g() { new const int; }"), Some("NEW-11"));
}

#[test]
fn unions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The members share the first byte; an assignment changes the active
    // member, and a copy keeps it.
    assert_eq!(run("union U { int a; float b; }; int main() { U u{1}; u.b = 2.0f; U v = u; return (int)v.b * 10 + (sizeof(U) == 4) + ((void*)&u.a == (void*)&u.b); }"), RunOutcome::Result(22));
    assert_eq!(run("struct P { int x, y; }; union U { int a; P p; }; int main() { U u{1}; u.p.y = 3; return u.p.y; }"), RunOutcome::Result(3));
    // A designator, a default member initializer, or a mem-init picks the
    // active member.
    assert_eq!(run("union U { int a; float b; }; union V { int a; float b = 1.5f; }; union W { int a; float b; W() : b(4) {} }; int main() { U u = {.b = 2.5f}; V v; W w; return (int)u.b * 100 + (int)(v.b * 2) * 10 + (int)w.b; }"), RunOutcome::Result(234));
    let r = run("union U { int a; float b; }; int main() { U u{1}; u.b = 2.0f; return u.a; }");
    assert!(matches!(r, RunOutcome::Undefined("UNI-6", ..)), "{:?}", r);
    let r = run("union U { int a; float b; }; int main() { U u; return u.a; }");
    assert!(matches!(r, RunOutcome::Undefined("UNI-6", ..)), "{:?}", r);
    assert_eq!(rule("union U { int a; float b; }; constexpr U u{.b = 2.0f}; constexpr U v = u; static_assert(v.b == 2.0f);"), None);
    assert_eq!(rule("union U { int a; float b; }; constexpr int f() { U u{1}; u.b = 2; return u.a; } static_assert(f() == 1);"), Some("SA-7"));
    assert_eq!(rule("union U { int& r; };"), Some("UNI-2"));
    assert_eq!(rule("union U { int a; float b; }; U u = {1, 2};"), Some("UNI-8"));
    assert_eq!(rule("union U { int a = 1; float b = 2; };"), Some("UNI-8"));
    assert_eq!(rule("union U { int a; float b; U() : a(1), b(2) {} };"), Some("UNI-8"));
    assert_eq!(rule("struct S; union S { int i; };"), Some("TYN-10"));
    assert_eq!(rule("void f() { union { int a; }; }"), Some("unsupported"));
}

#[test]
fn bit_fields() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A store wraps to the width; a narrow bit-field promotes to int.
    assert_eq!(run("struct B { int a : 3; unsigned b : 5; }; int main() { B x{3, 31}; x.a = 5; x.b = x.b + 1; return x.a * 100 + x.b; }"), RunOutcome::Result(-300));
    assert_eq!(run("struct B { unsigned x : 4; }; int main() { B b{15}; return (b.x - 16) < 0; }"), RunOutcome::Result(1));
    // BF-9: units of the type, and an unnamed width 0 starts a new one;
    // an unnamed bit-field takes no value.
    assert_eq!(run("struct B { char a; int b : 4; int : 0; int c : 3; }; struct C { int x : 4, : 4, y : 4; }; int main() { C c{1, 2}; return sizeof(B) * 10 + c.y; }"), RunOutcome::Result(82));
    assert_eq!(run("struct B { int x : 4; }; int main() { B b{7}; const int& r = b.x; b.x = 1; return r; }"), RunOutcome::Result(7));
    assert_eq!(rule("struct B { int x : 4; }; int* f(B& b) { return &b.x; }"), Some("BF-4"));
    assert_eq!(rule("struct B { int x : 4; }; int* f(B& b) { return &(b.x = 1); }"), Some("ASN-10"));
    assert_eq!(rule("struct B { int x : 4; }; int& f(B& b) { return b.x; }"), Some("BF-4"));
    assert_eq!(rule("struct B { int x : 4; }; auto n = sizeof(B{}.x);"), Some("BF-4"));
    assert_eq!(rule("struct B { float x : 4; };"), Some("BF-2"));
    assert_eq!(rule("struct B { int x : 0; };"), Some("BF-2"));
    assert_eq!(rule("struct B { int x : -1; };"), Some("BF-2"));
    let warned = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options { werror: true, ..Default::default() }).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(warned("struct B { unsigned char c : 12; };"), Some("BF-8"));
    assert_eq!(warned("struct B { unsigned x : 3; }; void f(B& b) { b.x = 9; }"), Some("BF-8"));
    assert_eq!(warned("struct B { unsigned x : 3; }; void f(B& b) { b.x = 7; }"), None);
    // A bit-field source counts by its width for narrowing.
    assert_eq!(rule("struct B { int x : 4; }; void f(B b) { char c{b.x}; (void)c; }"), None);
}

#[test]
fn conversion_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // An initialization, an argument, a built-in operator, a definition
    // outside the class, and a base's conversion function.
    assert_eq!(run("struct S { int v; operator int() const { return v; } }; int f(int x) { return x; } int main() { S s{5}; int a = s; return f(s) + a + (s + 1); }"), RunOutcome::Result(16));
    assert_eq!(run("struct S { operator int() const; }; S::operator int() const { return 4; } struct B { operator int() const { return 7; } }; struct D : B {}; int main() { S s; D d; return s * 10 + d; }"), RunOutcome::Result(47));
    // An explicit one: a condition and a static_cast, not a copy.
    assert_eq!(run("struct S { explicit operator bool() const { return true; } explicit operator int() const { return 3; } }; int main() { S s; if (s) return static_cast<int>(s); return 0; }"), RunOutcome::Result(3));
    assert_eq!(rule("struct S { explicit operator bool() const; }; void g() { S s; bool b = s; }"), Some("CVF-11"));
    // A switch picks the function by the object's cv-qualifiers.
    assert_eq!(run("struct S { operator int() const { return 1; } operator int() { return 2; } }; int pick(const S s) { switch (s) { case 1: return 5; default: return 9; } } int main() { return pick(S{}); }"), RunOutcome::Result(5));
    assert_eq!(rule("struct S { operator int(); }; const S s{}; int x = s;"), Some("INIT-7"));
    assert_eq!(rule("struct S { operator int() const; operator long() const; }; double d = S{};"), Some("CVF-12"));
    assert_eq!(rule("struct S { int operator int(); };"), Some("CVF-7"));
    assert_eq!(rule("struct S { operator int(int); };"), Some("CVF-7"));
    assert_eq!(rule("struct S { static operator int(); };"), Some("CVF-7"));
    let warned = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options { werror: true, ..Default::default() }).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(warned("struct S { operator S() const; };"), Some("CVF-6"));
}

#[test]
fn member_types() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A member alias, typedef, nested class, and enumeration, named with
    // the class's qualifier.
    assert_eq!(run("struct S { using T = int; typedef long L; struct N { int v; }; enum E { a, b }; }; int main() { S::T x = 2; S::L y = 3; S::N n{4}; return x + y + n.v + S::b; }"), RunOutcome::Result(10));
    // ACC-2: a private member type in a function body outside the class.
    assert_eq!(rule("class O { struct H { int v; }; public: static int f(); }; int g() { O::H h{1}; return h.v; }"), Some("ACC-2"));
    // ACC-19: the enclosing class has no access to the nested class's
    // private members.
    assert_eq!(rule("class Outer { class Inner { int secret = 1; }; int look(const Inner& i) { return i.secret; } };"), Some("ACC-2"));
    assert_eq!(rule("struct S { struct { int a; }; };"), Some("CLS-9"));
    assert_eq!(rule("struct S { using S = int; };"), Some("CLS-9"));
    assert_eq!(rule("struct S { using T = int; using T = int; };"), Some("TYN-2"));
}

#[test]
fn reference_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A reference member holds a pointer and names the object it refers
    // to; a copy refers to the same object.
    assert_eq!(run("int g = 4; struct S { int& r; }; int main() { S s{g}; s.r = 7; return g + (sizeof(S) == sizeof(void*)); }"), RunOutcome::Result(8));
    assert_eq!(run("struct S { int& r; S(int& x) : r(x) {} }; int main() { int a = 2; S s(a); S t = s; t.r = 9; return a; }"), RunOutcome::Result(9));
    assert_eq!(rule("struct S { const int& r; S() : r(5) {} };"), Some("CTOR-17"));
    assert_eq!(rule("struct S { int& r; }; void f() { S s; }"), Some("CTOR-20"));
    assert_eq!(rule("struct S { int& r; S() {} };"), Some("CTOR-3"));
    assert_eq!(rule("struct S { int& r; }; void f(int& a) { S s{a}, t{a}; s = t; }"), Some("CPY-23"));
    assert_eq!(rule("struct S { int&& r; }; void f(S& a) { S b(a); }"), Some("CPY-23"));
    assert_eq!(rule("int g; struct S { mutable int& r = g; };"), Some("CLS-9"));
    // DTY-1: an unparenthesized member access names the declared type.
    assert_eq!(rule("struct P { int first; decltype(static_cast<P*>(nullptr)->first) second; }; static_assert(sizeof(P) == 2 * sizeof(int));"), None);
}
