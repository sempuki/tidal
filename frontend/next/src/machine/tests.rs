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
    assert!(matches!(outcome("template <class T> struct S { int a = T::v; };\nstatic_assert(1 == 2);"), Outcome::Unsupported(_)));
    assert!(matches!(outcome("static_assert(1 == 2);\ntemplate <class T> struct S { int a = T::v; };"), Outcome::Reject(_)));
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
    assert_eq!(rule("int operator\"\"_k(unsigned long long); int x = 1_k;"), None);
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
    assert_eq!(rule("namespace std { class strong_ordering; } auto r = 1 <=> 2;"), Some("LSUP-2"));
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
    // A nested named union waits for a later part.
    assert_eq!(rule("struct A { union U { int a; } u; };"), Some("unsupported"));
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
    assert_eq!(rule("struct B {}; struct D : virtual B {};"), None);
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
    assert_eq!(rule("struct B { virtual void f(); }; struct D : virtual B {};"), None);
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
    assert_eq!(rule("void f() { union { int a; }; a = 1; }"), None);
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

#[test]
fn friends() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A friend function and a friend class read private members.
    assert_eq!(run("class C { int s = 4; friend int reveal(const C&); }; int reveal(const C& c) { return c.s; } int main() { return reveal(C{}); }"), RunOutcome::Result(4));
    assert_eq!(run("class C { int s = 6; friend class I; }; class I { public: int look(const C& c) const { return c.s; } }; int main() { return I{}.look(C{}); }"), RunOutcome::Result(6));
    // A friend defined in its class: argument-dependent lookup finds it,
    // and its body sees the class's members.
    assert_eq!(run("struct C { static const int k = 5; int v = 2; friend int get(C c) { return k + c.v; } }; int main() { return get(C{}); }"), RunOutcome::Result(7));
    assert_eq!(rule("class C { int s = 6; }; int f(const C& c) { return c.s; }"), Some("ACC-2"));
    assert_eq!(rule("namespace n { class C { friend int g(C) { return 1; } }; } int h() { return n::g(n::C{}); }"), Some("ACC-17"));
    assert_eq!(rule("namespace n { struct S {}; struct H { friend int probe(S) { return 1; } }; } int r() { return probe(n::S{}); }"), Some("NAM-8"));
    assert_eq!(rule("struct K { friend void open(K, int = 1); };"), Some("DEFARG-11"));
}

#[test]
fn anonymous_unions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A member of a class, of a block, and of a namespace.
    assert_eq!(run("struct V { int k; union { int i; float f; }; int get() { return i; } }; int main() { V v{1, 2}; v.i = v.get() + 3; return v.i * 10 + v.k; }"), RunOutcome::Result(51));
    assert_eq!(run("static union { int s; float t; }; int main() { union { int i; float f; }; i = 4; s = 2; return i + s; }"), RunOutcome::Result(6));
    // A designator and a mem-init name a member directly.
    assert_eq!(run("struct V { int k; union { int i; float f; }; V() : f(2.5f) {} }; struct W { int k; union { int i; float f; }; }; int main() { V v; W w{.f = 1.5f}; return (int)(v.f * 2) + (int)(w.f * 2); }"), RunOutcome::Result(8));
    let r = run("struct V { union { int i; float f; }; }; int main() { V v{3}; return (int)v.f; }");
    assert!(matches!(r, RunOutcome::Undefined("UNI-6", ..)), "{:?}", r);
    assert_eq!(rule("union { int i; };"), Some("UNI-9"));
    assert_eq!(rule("struct S { union { private: int x; }; };"), Some("UNI-9"));
    assert_eq!(rule("struct S { int i; union { int i; }; };"), Some("UNI-9"));
    assert_eq!(rule("struct S { union { int S; }; };"), Some("CLS-9"));
    assert_eq!(rule("struct S { union { int a; float b; }; S() : a(1), b(2) {} };"), Some("UNI-8"));
}

#[test]
fn ref_qualifiers() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // CLS-6: an lvalue object selects `&`, an rvalue `&&`.
    assert_eq!(run("struct S { int f() & { return 1; } int f() && { return 2; } }; int main() { S s; return s.f() * 10 + S{}.f(); }"), RunOutcome::Result(12));
    assert_eq!(run("struct S { operator int() & { return 1; } operator int() && { return 2; } }; int main() { S s; int a = s; int b = S{}; return a * 10 + b; }"), RunOutcome::Result(12));
    assert_eq!(rule("struct S { int f() && { return 2; } }; int g() { S s; return s.f(); }"), Some("OVL-35"));
    assert_eq!(rule("struct S { int f() &; int f(); };"), Some("CLS-26"));
    assert_eq!(rule("struct S { static int f() &; };"), Some("CLS-27"));
    assert_eq!(rule("struct B { virtual int f() &; }; struct D : B { int f() && override; };"), Some("VIRT-3"));
}

#[test]
fn member_default_arguments() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // DEFARG-3: a default argument sees members declared after it.
    assert_eq!(run("struct G { int level(int n = full) { return n; } static const int full = 9; }; int main() { G g; return g.level() + g.level(1); }"), RunOutcome::Result(10));
    // A constructor whose parameters have default arguments is a default
    // constructor, also for each element of an array.
    assert_eq!(run("struct R { int n, d; constexpr R(int a = 0, int b = 1) : n(a), d(b) {} }; struct S { R t; }; int main() { R p; R list[2]; S s; return p.d + list[1].d + s.t.d + R(5).n; }"), RunOutcome::Result(8));
    // VC-9: the temporaries of each element's default argument end with
    // the element.
    assert_eq!(run("int trace = 0; struct A { ~A() { trace = trace * 10 + 9; } }; struct E { E(const A& = A{}) { trace = trace * 10 + 1; } }; int main() { E arr[2]; return trace == 1919 ? 0 : 1; }"), RunOutcome::Result(0));
    assert_eq!(rule("struct S { void f(int a = 1, int b); };"), Some("DEFARG-8"));
}

#[test]
fn explicit_destructor_calls() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // DTOR-6: the destructor runs and the object's life ends.
    assert_eq!(run("int n = 0; struct S { ~S() { ++n; } }; int main() { S* p = new S; p->~S(); return n; }"), RunOutcome::Result(1));
    // DTOR-7: the end of the block destroys it again, also with a trivial
    // destructor.
    let r = run("int n = 0; struct S { ~S() { ++n; } }; int main() { { S s; s.~S(); } return n; }");
    assert!(matches!(r, RunOutcome::Undefined("DTOR-7", ..)), "{:?}", r);
    let r = run("struct S { int v; }; int main() { S s{1}; s.~S(); return 0; }");
    assert!(matches!(r, RunOutcome::Undefined("DTOR-7", ..)), "{:?}", r);
    assert_eq!(rule("struct B {}; struct D : B {}; void f(D d) { d.~B(); }"), Some("DTOR-8"));
    assert_eq!(rule("struct S {}; void f(S s) { s.~S(1); }"), Some("DTOR-8"));
}

#[test]
fn nested_class_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A nested class's member functions see the complete enclosing class
    // and its private members (ACC-10, CLS-2).
    assert_eq!(run("class Q { int items[3] = {4, 5, 6}; public: struct C { const Q* o; int at(int k) const { return o->items[k]; } }; C cursor() const { return {this}; } }; int main() { Q q; return q.cursor().at(1); }"), RunOutcome::Result(5));
    // DEFARG-3: and so do their default arguments.
    assert_eq!(run("struct O { struct I { int get(int n = depth) { return n; } }; static const int depth = 7; }; int main() { return O::I{}.get(); }"), RunOutcome::Result(7));
    // `struct L;` declares a nested class, which a later definition
    // completes.
    assert_eq!(run("struct O { struct L; struct I { L make() const { return L{4}; } }; struct L { int k; }; }; int main() { return O::I{}.make().k; }"), RunOutcome::Result(4));
    assert_eq!(rule("typedef struct { struct P { int size() const; } part; } W;"), Some("TYN-9"));
}

#[test]
fn pointers_to_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // MPTR-3: `.*` and `->*` reach the member of the object, and a call
    // through a pointer to a virtual function calls the final overrider.
    assert_eq!(run("struct S { int a = 1, b = 2; }; int main() { S s; int S::* p = &S::b; S* q = &s; q->*p = 5; return s.*p; }"), RunOutcome::Result(5));
    assert_eq!(run("struct B { virtual int f() { return 1; } }; struct D : B { int f() override { return 2; } }; int main() { int (B::* pf)() = &B::f; D d; B& b = d; return (b.*pf)(); }"), RunOutcome::Result(2));
    // MPTR-4, MPTR-5: a conversion to a derived class keeps the path to
    // the member's subobject, so two paths give unequal pointers.
    assert_eq!(run("struct A { int x; }; struct B : A {}; struct C : A {}; struct D : B, C {}; int main() { int B::* pb = &A::x; int C::* pc = &A::x; int D::* p = pb; int D::* q = pc; return p != q; }"), RunOutcome::Result(1));
    // MPTR-6: an object without the member.
    let r = run("struct B { int x = 0; }; struct D : B { int y = 5; }; int main() { B b; int B::* pm = static_cast<int B::*>(&D::y); return b.*pm; }");
    assert!(matches!(r, RunOutcome::Undefined("MPTR-6", ..)), "{:?}", r);
    assert_eq!(rule("struct S { int m; }; int S::* p = &S::m; bool b = p < p;"), Some("MPTR-12"));
    assert_eq!(rule("struct S { int f(); }; int g(S s) { int (S::* p)() = &S::f; return s.*p(); }"), Some("MPTR-10"));
    assert_eq!(rule("struct B { int m; }; struct D : B {}; int B::* p = static_cast<int D::*>(nullptr); int D::* q = &B::m; int B::* r = q;"), Some("INIT-7"));
}

#[test]
fn alignment_of_classes_and_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // ALN-1: `alignas` on a class and on a member raises the alignment,
    // also of a class that derives from it.
    assert_eq!(rule("struct alignas(32) L { char d[4]; }; static_assert(alignof(L) == 32 && sizeof(L) == 32);"), None);
    assert_eq!(rule("struct S { char c; alignas(8) int v; }; static_assert(sizeof(S) == 16);"), None);
    assert_eq!(rule("struct alignas(32) T {}; struct H : T { char x; }; static_assert(alignof(H) == 32);"), None);
    // ALN-4: an object's address is a multiple of its alignment.
    assert_eq!(run("struct alignas(256) B { int v; }; int main() { B b; return reinterpret_cast<unsigned long>(&b) % 256; }"), RunOutcome::Result(0));
    assert_eq!(rule("struct alignas(1) S { int v; };"), Some("ALN-2"));
    assert_eq!(rule("struct S { alignas(4) int b : 3; };"), Some("ALN-2"));
    assert_eq!(rule("struct alignas(1 << 29) H { char c; };"), Some("ALN-5"));
}

#[test]
fn using_declarations_in_classes() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // DER-3: a using-declaration brings the base's overloads, and a
    // function of the class with the same parameters hides one.
    assert_eq!(run("struct B { int f(int) { return 1; } }; struct D : B { using B::f; int f(const char*) { return 2; } }; int main() { D d; return d.f(0) * 10 + d.f(\"x\"); }"), RunOutcome::Result(12));
    assert_eq!(run("struct B { int f(int) { return 1; } }; struct D : B { using B::f; int f(int) { return 2; } }; int main() { D d; return d.f(0); }"), RunOutcome::Result(2));
    // ACC-9: the member has the access of the using-declaration.
    assert_eq!(rule("class B { protected: int x = 1; }; class D : public B { public: using B::x; }; int f(D& d) { return d.x; }"), None);
    assert_eq!(rule("struct B { int x; }; class D : public B { using B::x; }; int f(D& d) { return d.x; }"), Some("ACC-2"));
    assert_eq!(rule("class B { int x; }; class D : public B { public: using B::x; };"), Some("ACC-13"));
    assert_eq!(rule("struct X { int f(); }; struct B {}; struct D : B { using X::f; };"), Some("DER-9"));
}

#[test]
fn inheriting_constructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // DER-8: the base's constructors construct the derived class, and the
    // other members take their default member initializers.
    assert_eq!(run("struct B { int v; B(int x, int y = 10) : v(x + y) {} }; struct D : B { using B::B; int w = 9; }; int main() { D a(1); return a.v + a.w; }"), RunOutcome::Result(20));
    // OVL-31: a constructor of the derived class beats an inherited one.
    assert_eq!(run("struct B { int tag = 0; B(int) : tag(1) {} }; struct D : B { using B::B; D(int, int = 0) : B(0) { tag = 2; } }; int main() { return D(5).tag; }"), RunOutcome::Result(2));
    // OVL-17: an inherited constructor of the copy's shape is not one.
    assert_eq!(rule("struct B { B() {} B(const B&, int = 0) {} }; struct D : B { using B::B; }; B b; D d(b);"), Some("CTOR-25"));
    assert_eq!(rule("struct A { A(int) {} }; struct B : A { using A::A; }; struct D : B { using A::A; };"), Some("DER-9"));
    assert_eq!(rule("class B { B(int) {} public: B() {} friend struct D; }; struct D : B { using B::B; }; D d(1);"), Some("DER-20"));
}

#[test]
fn const_objects_without_initializers() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // CTOR-4: default-initialization gives each member a value.
    assert_eq!(rule("struct S { int v = 1; }; const S s;"), None);
    assert_eq!(rule("struct B { int v = 0; }; struct D : B {}; const D d;"), None);
    assert_eq!(rule("struct S { int v; }; const S s;"), Some("CTOR-4"));
    assert_eq!(rule("struct S { S() = default; int v; }; const S s;"), Some("CTOR-4"));
    // CTOR-24: an anonymous union member with no default member
    // initializer.
    assert_eq!(rule("struct C { int tag = 1; union { int w; float f; }; }; const C c;"), Some("CTOR-24"));
    // CLS-5: a mutable member of a const object can change.
    assert_eq!(run("struct C { mutable int hits = 0; int get() const { return ++hits; } }; int main() { const C c; c.get(); return c.get(); }"), RunOutcome::Result(2));
    // CE-23: an implicit destructor is constexpr when those of its bases
    // are.
    assert_eq!(rule("struct B { constexpr virtual ~B() {} }; struct D : B { int v = 1; }; constexpr D d;"), None);
}

#[test]
fn structured_bindings_of_classes() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // SB-3: the names bind the data members, also those of a base.
    assert_eq!(run("struct B { int a; int b; }; struct D : B {}; int main() { D d{{3, 4}}; auto& [x, y] = d; y = 9; return x * 10 + d.b; }"), RunOutcome::Result(39));
    // SB-6: a const reference keeps the temporary alive.
    assert_eq!(run("struct T { int a; int b; }; int main() { const auto& [x, y] = T{1, 2}; return x + y; }"), RunOutcome::Result(3));
    assert_eq!(rule("struct P { int x; int y; }; void f(P p) { auto [a] = p; }"), Some("SB-2"));
    assert_eq!(rule("struct B { int a; }; struct D : B { int b; }; void f(D d) { auto [x, y] = d; }"), Some("SB-13"));
    assert_eq!(rule("class P { int x; }; void f(P p) { auto [a] = p; }"), Some("SB-13"));
    assert_eq!(rule("union U { int a; }; void f(U u) { auto [a] = u; }"), Some("SB-13"));
}

#[test]
fn dynamic_casts() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // VIRT-10: a cast down, a cross cast, and a failed cast to a null
    // pointer.
    assert_eq!(run("struct B { virtual ~B() {} }; struct D : B { int v = 3; }; int main() { D d; B* b = &d; return dynamic_cast<D*>(b)->v; }"), RunOutcome::Result(3));
    assert_eq!(run("struct A { virtual ~A() {} }; struct B { virtual ~B() {} }; struct D : A, B {}; int main() { D d; A* a = &d; return dynamic_cast<B*>(a) == static_cast<B*>(&d) ? 0 : 1; }"), RunOutcome::Result(0));
    assert_eq!(run("struct B { virtual ~B() {} }; struct D : B {}; struct E : B {}; int main() { E e; B* b = &e; return dynamic_cast<D*>(b) == nullptr ? 0 : 1; }"), RunOutcome::Result(0));
    // During B's constructor the object is a B.
    assert_eq!(run("struct D; bool f(struct B*); struct B { bool s; B() { s = f(this); } virtual ~B() {} }; struct D : B {}; bool f(B* p) { return dynamic_cast<D*>(p) == nullptr; } int main() { D d; return d.s ? 0 : 1; }"), RunOutcome::Result(0));
    assert_eq!(rule("struct B {}; struct D : B {}; D* f(B* b) { return dynamic_cast<D*>(b); }"), Some("VIRT-18"));
    assert_eq!(rule("struct B { virtual ~B() {} }; int* f(B* b) { return dynamic_cast<int*>(b); }"), Some("VIRT-18"));
    assert_eq!(rule("struct B { constexpr virtual ~B() {} }; struct D : B {}; constexpr D d; constexpr const B& r = d; static_assert(&dynamic_cast<const D&>(r) == &d);"), None);
}

#[test]
fn virtual_bases_in_checks_and_layout() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // DER-6: two paths reach one virtual base, so the conversion and the
    // member are unambiguous; CLS-35: the virtual base comes last.
    assert_eq!(rule("struct A { int v; }; struct B : virtual A {}; struct C : virtual A {}; struct D : B, C {}; A* f(D* d) { d->v = 1; return d; } static_assert(sizeof(B) == 16 && sizeof(D) == 24);"), None);
    assert_eq!(rule("struct A {}; struct B : virtual A {}; struct C : A {}; struct D : B, C {}; A* f(D* d) { return d; }"), Some("DER-10"));
    assert_eq!(rule("struct A {}; struct B : virtual A {}; B* f(A* a) { return static_cast<B*>(a); }"), Some("DER-11"));
    assert_eq!(rule("struct A {}; struct D : virtual A { constexpr D() {} };"), Some("DER-12"));
    // DER-13: the most derived class initializes the virtual base.
    assert_eq!(rule("struct A { A(int) {} }; struct B : virtual A { B() : A(1) {} }; struct D : B { D() : A(2) {} };"), None);
    assert_eq!(rule("struct A { A(int) {} }; struct B : virtual A { B() : A(1) {} }; struct D : B { D() {} };"), Some("DER-13"));
    assert_eq!(rule("struct A { virtual void f(); }; struct B : virtual A { void f() override; }; struct C : virtual A { void f() override; }; struct D : B, C {};"), Some("VIRT-5"));
}

#[test]
fn virtual_bases_at_run_time() {
    // DER-6: one subobject, which both paths reach.
    assert_eq!(run("struct A { int v = 0; }; struct B : virtual A {}; struct C : virtual A {}; struct D : B, C {}; int main() { D d; static_cast<B&>(d).v = 5; return static_cast<C&>(d).v; }"), RunOutcome::Result(5));
    // DER-7: the virtual base first and once, from the most derived
    // class's initializer; destruction in reverse.
    assert_eq!(run("int t = 0; void mark(int d) { t = t * 10 + d; } struct V { int v; V(int x) : v(x) { mark(1); } ~V() { mark(1); } }; struct B { B() { mark(2); } ~B() { mark(2); } }; struct C : virtual V { C() : V(7) { mark(3); } ~C() { mark(3); } }; struct D : B, C { D() : V(9) { mark(4); } ~D() { mark(4); } }; int main() { int v; { D d; v = d.v; } return t == 12344321 ? v : 0; }"), RunOutcome::Result(9));
    // VIRT-5: the final overrider, from a pointer to the shared base.
    assert_eq!(run("struct A { virtual int f() { return 1; } }; struct B : virtual A { int f() override { return 2; } }; struct C : virtual A {}; struct D : B, C {}; int main() { D d; A& r = d; C& c = d; return r.f() * 10 + c.f(); }"), RunOutcome::Result(22));
    // A copy of a complete object copies its virtual base.
    assert_eq!(run("struct A { int v = 1; }; struct B : virtual A { int w = 2; }; int main() { B x; x.v = 6; B y = x; return y.v + y.w; }"), RunOutcome::Result(8));
}

#[test]
fn byte_offsets_through_virtual_bases() {
    // STG-13: a byte pointer reaches the virtual base at its offset in
    // the complete object.
    assert_eq!(run("struct V { int v = 7; }; struct A : virtual V { int a = 2; }; int main() { A x; char* p = reinterpret_cast<char*>(&x); V* q = &x; int off = reinterpret_cast<char*>(q) - p; int* r = reinterpret_cast<int*>(p + off); return off + *r; }"), RunOutcome::Result(19));
}

#[test]
fn allocation_functions_and_placement() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    const PLACE: &str = "inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; } ";
    // STG-5: placement new creates the object in a byte array, at its
    // address.
    assert_eq!(run(&format!("{PLACE}struct P {{ int x, y; P(int a, int b) : x(a), y(b) {{}} }}; int main() {{ alignas(P) unsigned char b[sizeof(P)]; P* p = new (b) P(3, 4); bool same = static_cast<void*>(p) == b; return same ? p->x + p->y : 0; }}")), RunOutcome::Result(7));
    // NEW-13: a class's own functions; NEW-19: the global ones by name.
    assert_eq!(run("int n = 0; struct S { int v = 5; static void* operator new(decltype(sizeof 0) k) { ++n; return ::operator new(k); } static void operator delete(void* p) { n += 10; ::operator delete(p); } }; int main() { S* s = new S; int v = s->v; delete s; return n * 10 + v; }"), RunOutcome::Result(115));
    // STG-8: storage from operator new holds objects of implicit-lifetime
    // types, and its elements step as an array.
    assert_eq!(run("int main() { int* p = static_cast<int*>(::operator new(4 * sizeof(int))); p[2] = 5; int v = p[2]; ::operator delete(p); return v; }"), RunOutcome::Result(5));
    assert_eq!(run("int main() { void* p = ::operator new[](8); ::operator delete(p); return 0; }"), RunOutcome::Undefined("NEW-40", "a deallocation by the form that does not match the allocation".into(), 1));
    assert_eq!(rule("struct S { private: static void* operator new(decltype(sizeof 0)); }; S* f() { return new S; }"), Some("NEW-25"));
    assert_eq!(rule("struct T {}; T* f() { return new (1) T; }"), Some("NEW-20"));
}

#[test]
fn exceptions_throw_and_unwind() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // TRY-1, TRY-2: the first handler that matches, a base by reference.
    assert_eq!(run("struct B { int v; }; struct D : B {}; int main() { try { throw D{{7}}; } catch (int) { return 1; } catch (B& b) { return b.v; } return 2; }"), RunOutcome::Result(7));
    // UNW-1: the objects of the blocks that the exception leaves end
    // before the handler runs.
    assert_eq!(run("int t = 0; struct S { int d; ~S() { t = t * 10 + d; } }; void f() { S a{1}; S b{2}; throw 0; } int main() { try { f(); } catch (int) { t = t * 10 + 3; } return t; }"), RunOutcome::Result(213));
    // THR-6: a rethrow passes on the same object.
    assert_eq!(run("int main() { try { try { throw 5; } catch (int& e) { e = 9; throw; } } catch (int e) { return e; } return 0; }"), RunOutcome::Result(9));
    // UNW-2: a constructor that throws destroys the members that it built.
    assert_eq!(run("int t = 0; struct M { M() {} ~M() { ++t; } }; struct S { M a; S() { throw 1; } }; int main() { try { S s; } catch (int) {} return t; }"), RunOutcome::Result(1));
    // UNW-5: an exception that leaves a noexcept function ends the
    // program.
    assert_eq!(run("void f() noexcept { throw 1; } int main() { try { f(); } catch (...) {} return 0; }"), RunOutcome::Result(134));
    assert_eq!(rule("void f() { try {} catch (...) {} catch (int) {} }"), Some("TRY-3"));
    assert_eq!(rule("struct I; void f(I* p) { throw *p; }"), Some("THR-3"));
}

#[test]
fn three_way_and_defaulted_comparisons() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    const ORD: &str = "namespace std { struct strong_ordering { signed char v; static const strong_ordering less, equal, equivalent, greater; friend constexpr bool operator==(strong_ordering, strong_ordering) = default; friend constexpr bool operator==(strong_ordering s, int) { return s.v == 0; } friend constexpr bool operator<(strong_ordering s, int) { return s.v < 0; } friend constexpr bool operator<(int, strong_ordering s) { return 0 < s.v; } }; inline constexpr strong_ordering strong_ordering::less{-1}; inline constexpr strong_ordering strong_ordering::equal{0}; inline constexpr strong_ordering strong_ordering::equivalent{0}; inline constexpr strong_ordering strong_ordering::greater{1}; } ";
    // CMP-27: a built-in <=> copies a static data member.
    assert_eq!(run(&format!("{ORD}int main() {{ return (1 <=> 2) < 0 ? 0 : 1; }}")), RunOutcome::Result(0));
    // DCMP-1, DCMP-4: defaulted == and <=> compare the members in order;
    // a relational operator is rewritten from <=>, and a defaulted <=>
    // declares ==.
    assert_eq!(run(&format!("{ORD}struct P {{ int x; int y; auto operator<=>(const P&) const = default; }}; int main() {{ P a{{3, 4}}, b{{3, 5}}; return (a < b) && !(b < a) && a != b && a == P{{3, 4}} ? 0 : 1; }}")), RunOutcome::Result(0));
    assert_eq!(rule(&format!("{ORD}static_assert(0 < (2 <=> 1));")), None);
    // DCMP-2: a reference member deletes the defaulted ==.
    assert_eq!(rule("struct R { int& r; bool operator==(const R&) const = default; }; bool f(const R& a) { return a == a; }"), Some("DEL-1"));
    assert_eq!(rule("struct S { int x; bool operator==(S) const = default; };"), Some("DCMP-3"));
    assert_eq!(rule("auto r = 1 <=> 2;"), Some("CMP-25"));
}

#[test]
fn function_try_blocks() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // TRY-9: the handlers see the parameters, and a return gives the
    // result.
    assert_eq!(run("int f(int x) try { throw 1; } catch (int) { return x + 1; } int main() { return f(4); }"), RunOutcome::Result(5));
    // TRY-10: a constructor's handler sees a member initializer's
    // exception and rethrows it at its end.
    assert_eq!(run("struct M { M(int v) { if (v == 0) throw 7; } }; struct S { M m; S(int v) try : m(v) {} catch (int) {} }; int main() { try { S s(0); } catch (int e) { return e; } return 0; }"), RunOutcome::Result(7));
    assert_eq!(rule("struct S { S() try {} catch (...) { return; } };"), Some("TRY-17"));
}

#[test]
fn literal_operators() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // OPR-8: the numeric form wins over the raw form, which takes the
    // characters of the literal.
    assert_eq!(run("int operator\"\"_w(unsigned long long v) { return int(v); } int operator\"\"_w(const char* s) { return s[0]; } int operator\"\"_r(const char* s) { int n = 0; while (s[n]) ++n; return n; } int main() { return 12_w + 1.25_r; }"), RunOutcome::Result(16));
    // A string literal passes its characters and their count.
    assert_eq!(run("using Size = decltype(sizeof 0); Size operator\"\"_n(const char*, Size n) { return n; } int main() { return int(\"ab\" \"cde\"_n); }"), RunOutcome::Result(5));
    assert_eq!(rule("unsigned long long operator\"\"_u(unsigned long long n) { return n; } auto x = 1.5_u;"), Some("OPR-16"));
    assert_eq!(rule("int operator\"\"_b(int);"), Some("OPR-10"));
}

#[test]
fn explicit_object_parameters() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // CLS-13: the object of the call initializes the parameter, and the
    // overloads differ by its type.
    assert_eq!(run("struct V { int v = 3; int f(this V& s) { return s.v; } int f(this const V& s) { return s.v * 10; } }; int main() { V v; const V& c = v; return v.f() + c.f(); }"), RunOutcome::Result(33));
    // CLS-17: the address is an ordinary pointer to function.
    assert_eq!(run("struct S { int v; int get(this const S& s, int k) { return s.v + k; } }; int main() { int (*p)(const S&, int) = &S::get; return p(S{4}, 1); }"), RunOutcome::Result(5));
    assert_eq!(rule("struct S { int get(this S self) const; };"), Some("CLS-14"));
    assert_eq!(rule("struct S { int v; int get(this S& s) { return this->v; } };"), Some("PRIM-20"));
    assert_eq!(rule("struct S { void f(); void f(this S&); };"), Some("CLS-26"));
}

#[test]
fn noexcept_on_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // EXS-1: a noexcept operand of a member function sees the complete
    // class, so it can name a later member.
    assert_eq!(rule("struct S { void f() noexcept(noexcept(g())); void g() noexcept; }; static_assert(noexcept(S{}.f()));"), None);
    assert_eq!(rule("struct S { void f() noexcept(sizeof(S) > 64); int a; }; static_assert(!noexcept(S{}.f()));"), None);
    assert_eq!(rule("struct B { virtual void f() noexcept; }; struct D : B { void f() override; };"), Some("EXS-3"));
}

#[test]
fn union_special_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // UNI-3: a variant member with a non-trivial function deletes the
    // union's implicit one, and a use is an error (UNI-11).
    assert_eq!(rule("struct S { ~S() {} }; union U { S s; int i; U() : i(0) {} }; void f() { U u; }"), Some("UNI-11"));
    assert_eq!(rule("struct S { S() {} S(const S&) {} }; union U { S s; int i; U() : i(0) {} }; U a; U b(a);"), Some("UNI-11"));
    assert_eq!(rule("struct S { S() {} ~S() {} }; union U { S s; int i; U() : i(0) {} ~U() {} }; U u;"), None);
    // A default member initializer gives the union a default constructor.
    assert_eq!(rule("struct S { S() {} ~S() {} }; union U { int i = 0; S s; ~U() {} }; void f() { U u; }"), None);
    // UNI-5: an implicit assignment copies the active member.
    assert_eq!(run("struct P { P() = default; P(const P&) = default; P& operator=(const P&) = default; P& operator=(P&&) { return *this; } }; union U { P p; int n; }; int main() { U a; a.n = 4; U c; c = static_cast<U&&>(a); return c.n; }"), RunOutcome::Result(4));
}

#[test]
fn destructor_exceptions() {
    // DTOR-4: noexcept(false) lets an exception leave a destructor, and
    // the members still end (UNW-2).
    assert_eq!(run("int ended = 0; struct M { ~M() { ++ended; } }; struct S { M m; ~S() noexcept(false) { throw 3; } }; int main() { try { S s; } catch (int v) { return v * 10 + ended; } return 0; }"), RunOutcome::Result(31));
    // A destructor without a specification is non-throwing.
    assert_eq!(run("struct S { ~S() { throw 1; } }; int main() { try { S s; } catch (int) { return 1; } return 2; }"), RunOutcome::Result(134));
    // UNW-5: a destructor that throws during unwinding ends the program.
    assert_eq!(run("struct S { ~S() noexcept(false) { throw 1; } }; int main() { try { S s; throw 2; } catch (int) { return 1; } return 2; }"), RunOutcome::Result(134));
}

#[test]
fn range_for_over_classes() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // RFOR-2: member begin and end, an iterator class, and an end of
    // another type.
    assert_eq!(run("struct It { int v; int operator*() const { return v; } It& operator++() { ++v; return *this; } bool operator!=(int e) const { return v != e; } }; struct R { It begin() { return {1}; } int end() { return 4; } }; int main() { int s = 0; for (int x : R{}) s += x; return s; }"), RunOutcome::Result(6));
    // Free functions that argument-dependent lookup finds.
    assert_eq!(run("namespace n { struct B { int d[2] = {3, 4}; }; int* begin(B& b) { return b.d; } int* end(B& b) { return b.d + 2; } } int main() { n::B b; int s = 0; for (int& x : b) s += x; return s; }"), RunOutcome::Result(7));
    assert_eq!(rule("struct R { int d[2]; int* begin() { return d; } }; int* end(R& r) { return r.d + 2; } void f(R r) { for (int v : r) (void)v; }"), Some("RFOR-7"));
}

#[test]
fn local_class_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // CLS-12: a member function of a local class reads a constant of
    // the enclosing function as a value, and a static local.
    assert_eq!(run("int main() { const int k = 4; static int s = 2; struct P { int f() { return k + s; } }; return P{}.f(); }"), RunOutcome::Result(6));
    assert_eq!(rule("int f() { int local = 1; struct S { int get() { return local; } }; return S{}.get(); }"), Some("CLS-12"));
    assert_eq!(rule("void f() { const int k = 1; struct S { const int* p() { return &k; } }; }"), Some("CLS-12"));
}

#[test]
fn no_unique_address_layout() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // ATTR-9: empty members take no space, two of one type take two
    // addresses, and a later member can start at 0.
    assert_eq!(rule("struct E {}; struct T { [[no_unique_address]] E a; [[no_unique_address]] E b; int v; }; static_assert(sizeof(T) == sizeof(int)); struct O { [[no_unique_address]] E a; [[no_unique_address]] E b; }; static_assert(sizeof(O) == 2);"), None);
    // The tail padding of a member that is not a POD holds what follows.
    assert_eq!(rule("struct B { B() {} int a; char b; }; struct H { [[no_unique_address]] B part; char c; }; static_assert(sizeof(H) == 8);"), None);
    assert_eq!(rule("struct S { [[no_unique_address]] int b : 3; };"), Some("ATTR-2"));
}

#[test]
fn conditional_class_operands() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // COND-16: one operand converts to the other's type.
    assert_eq!(run("struct D { int m; D(int x) : m(x) {} }; int main() { D s = 20; D d = true ? 300 : s; return d.m == 300 ? 0 : 1; }"), RunOutcome::Result(0));
    // COND-11: two classes reach int by their conversion functions.
    assert_eq!(run("struct A { operator int() const { return 1; } }; struct B { operator int() const { return 2; } }; int main() { A a; B b; return false ? a : b; }"), RunOutcome::Result(2));
    assert_eq!(rule("struct M { M(int); operator int() const; }; M m(0); extern bool c; auto r = c ? m : 1;"), Some("COND-5"));
}

#[test]
fn array_member_initializers() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // CTOR-21: braces give the elements in order, and the rest are zero.
    assert_eq!(run("struct S { int a[3]; S() : a{4, 5} {} }; int main() { S s; return s.a[0] + s.a[1] + s.a[2]; }"), RunOutcome::Result(9));
    assert_eq!(rule("struct S { int p[2]; S(double d) : p{1, d} {} };"), Some("CTOR-21"));
    assert_eq!(rule("struct S { int a[2]; S(const int (&x)[2]) : a(x) {} };"), Some("INIT-8"));
}

#[test]
fn member_access_forms() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A qualified member name, a qualified call without dispatch, an
    // enumerator, and a parenthesized member call.
    assert_eq!(run("struct B { int b = 4; virtual int f() { return 1; } }; struct D : B { enum { k = 2 }; int f() override { return 3; } }; int main() { D d; B& r = d; return d.B::b + r.B::f() + d.k + (d.f)(); }"), RunOutcome::Result(10));
    // A conversion function called by its name.
    assert_eq!(run("struct S { operator int() const { return 3; } }; int main() { S s; return s.operator int(); }"), RunOutcome::Result(3));
    assert_eq!(rule("struct S { int f() { return 1; } }; void g(S s) { auto p = s.f; }"), Some("PFX-20"));
}

#[test]
fn builtin_operators_against_operator_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The operator function takes the class operand without a conversion
    // function, so it is better than the built-in operator.
    assert_eq!(run("struct S { operator int() const { return 1; } }; int operator+(S, int) { return 7; } int main() { S s; return s + 1; }"), RunOutcome::Result(7));
    // No operator function is viable, so the built-in operator is used.
    assert_eq!(run("struct S { operator int() const { return 2; } }; struct T {}; int operator+(T, T) { return 0; } int main() { S s; return s + 1; }"), RunOutcome::Result(3));
    // Each candidate needs a user-defined conversion on a different operand.
    assert_eq!(rule("struct S { S(int); operator int() const; }; int operator+(S, S); int f(S s) { return s + 1; }"), Some("OVL-36"));
    // Copy-initialization from another class.
    assert_eq!(rule("struct T { T(int); }; struct S { explicit operator T() const; }; S s; T t = s;"), Some("OVL-35"));
    assert_eq!(rule("struct A { operator int() const; }; struct B { B(int); }; A a; B b = a;"), Some("CVF-10"));
    // A const object is not const while its constructor runs.
    assert_eq!(run("struct S { int x; S() { x = 3; } }; int main() { const S s; return s.x; }"), RunOutcome::Result(3));
}

#[test]
fn reference_members_keep_temporaries() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The temporary lives as long as the aggregate variable and ends
    // after it.
    assert_eq!(run("int t = 0; struct N { int v; ~N() { t = t * 10 + v; } }; struct H { const N& n; ~H() { t = t * 10 + 9; } }; void f() { H h{N{5}}; t = h.n.v; } int main() { f(); return t; }"), RunOutcome::Result(595));
    // A temporary in an expression of the initializer ends with it.
    assert_eq!(run("int t = 0; struct N { int v; ~N() { ++t; } }; struct H { const N& n; }; int main() { int x = H{N{5}}.n.v; return t * 10 + x; }"), RunOutcome::Result(15));
    // A block-scope constexpr aggregate binds an automatic temporary.
    assert_eq!(rule("struct H { const int& n; }; void f() { constexpr H h{4}; }"), Some("CX-2"));
    assert_eq!(rule("struct H { const int& n; }; constexpr H h{4}; static_assert(h.n == 4);"), None);
}

#[test]
fn surrogate_call_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A call through a conversion function to a pointer to function.
    assert_eq!(run("int add(int a, int b) { return a + b; } using F = int (*)(int, int); struct S { operator F() const { return add; } }; int main() { S s; return s(2, 3); }"), RunOutcome::Result(5));
    // The call operator takes the object without a conversion.
    assert_eq!(run("int other(int) { return 9; } using F = int (*)(int); struct S { int operator()(int v) { return v; } operator F() { return other; } }; int main() { S s; return s(4); }"), RunOutcome::Result(4));
    // A conversion function qualified && does not serve an lvalue.
    assert_eq!(rule("using F = int (*)(int); struct S { operator F() &&; }; int f(S s) { return s(1); }"), Some("OVL-35"));
    // The call operator wins on the object and the surrogate on the argument.
    assert_eq!(rule("using F = int (*)(int); struct S { int operator()(long); operator F(); }; int f(S s) { return s(1); }"), Some("OVL-36"));
}

#[test]
fn braced_list_arguments() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The list copy-list-initializes a class, an aggregate, a scalar,
    // and the object a reference binds.
    assert_eq!(run("struct P { int x, y; P(int a, int b) : x(a), y(b) {} }; struct A { int a, b; }; int f(P p, A a, int i, const P& r) { return p.x + p.y + a.a + a.b + i + r.y; } int main() { return f({1, 2}, {3, 4}, {5}, {0, 6}); }"), RunOutcome::Result(21));
    // An explicit constructor takes no part in the copy.
    assert_eq!(rule("struct S { explicit S(int, int); }; void f(S); void g() { f({1, 2}); }"), Some("PFX-16"));
    // A list adds no associated namespace.
    assert_eq!(rule("namespace n { struct S {}; int probe(S); } int g() { return probe({}); }"), Some("NAM-8"));
}

#[test]
fn explicit_conditions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct S { explicit(false) S(int) {} }; S s = 1;"), None);
    assert_eq!(rule("struct S { explicit(true) S(int) {} }; S s = 1;"), Some("CTOR-9"));
    assert_eq!(rule("struct S { explicit(2) S(int); };"), Some("CE-19"));
    assert_eq!(rule("bool b = true; struct S { explicit(b) S(int); };"), Some("CTOR-9"));
    assert_eq!(rule("struct S { S(int); }; explicit S::S(int) {}"), Some("CTOR-9"));
    // A plain argument and a braced one copy-initialize the parameter.
    assert_eq!(rule("struct S { explicit S(int); }; void f(S); void g() { f(1); }"), Some("PFX-16"));
}

#[test]
fn arrow_operators() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A chain of operator-> calls ends at a pointer to a class, for member
    // access and for a member call.
    assert_eq!(run("struct T { int v = 6; int get() { return v + 1; } }; struct I { T t; T* operator->() { return &t; } }; struct O { I i; I& operator->() { return i; } }; int main() { O o; return o->v + o->get(); }"), RunOutcome::Result(13));
    assert_eq!(rule("struct S { int operator->(); }; int f(S s) { return s->x; }"), Some("OPR-14"));
    assert_eq!(rule("struct A { int x; A& operator->(); }; int f(A& a) { return a->x; }"), Some("OPR-14"));
    assert_eq!(rule("struct S { int v; }; int f(S s) { return s->v; }"), Some("PFX-11"));
    assert_eq!(rule("struct S {}; int* operator->(S);"), Some("OPR-2"));
}

#[test]
fn bit_field_widths_of_class_type() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct W { constexpr explicit operator int() const { return 3; } constexpr operator long() const { return 4; } }; struct S { unsigned f : W{}; }; constexpr S s{15}; static_assert(s.f == 15);"), None);
    assert_eq!(rule("struct W { operator int() const { return 3; } }; struct S { int b : W(); };"), Some("BF-2"));
    assert_eq!(rule("struct W { constexpr operator int() const { return 3; } constexpr operator long() const { return 3; } }; struct S { int b : W(); };"), Some("BF-2"));
    assert_eq!(rule("struct W { ~W() {} constexpr operator int() const { return 3; } }; struct S { int b : W(); };"), Some("BF-2"));
}

#[test]
fn thread_local_and_extern_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct C { static thread_local int total; }; thread_local int C::total = 10; int main() { C::total += 1; return C::total; }"), RunOutcome::Result(11));
    assert_eq!(rule("struct S { extern int v; };"), Some("CLS-9"));
    assert_eq!(rule("struct S { thread_local int v; };"), Some("CLS-9"));
}

#[test]
fn attributes_on_a_class() {
    let werror = |src: &str| {
        let o = crate::Options { werror: true, ..Default::default() };
        match crate::check_source(src, Path::new("t.cpp"), &o).0 {
            Outcome::Reject(ds) => ds[0].rule,
            Outcome::Accept => None,
            Outcome::Unsupported(_) => Some("unsupported"),
        }
    };
    // A call that returns a nodiscard class by value warns when discarded;
    // one that returns a reference does not.
    assert_eq!(werror("struct [[nodiscard]] E { int c; }; E f() { return {0}; } void g() { f(); }"), Some("ATTR-5"));
    assert_eq!(werror("struct [[nodiscard]] E { int c; }; E e; E& f() { return e; } void g() { f(); }"), None);
    assert_eq!(werror("struct [[noreturn]] S {};"), Some("ATTR-2"));
    // A vendor attribute does not change the layout.
    assert_eq!(werror("struct [[gnu::packed]] R { char k; int v; }; static_assert(sizeof(R) == 8);"), None);
}

#[test]
fn classes_with_qualified_names() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The definition sees the members of the qualifier's namespace.
    assert_eq!(run("namespace lib { struct Root { int r = 1; }; using len = int; struct Leaf; } struct lib::Leaf : Root { len x = 4; }; int main() { lib::Leaf l; return l.r + l.x; }"), RunOutcome::Result(5));
    assert_eq!(rule("struct P { union F; }; union P::F final { short h; }; static_assert(sizeof(P::F) == sizeof(short));"), None);
    assert_eq!(rule("struct O { struct I; }; struct O::M { int v; };"), Some("TYN-7"));
}

#[test]
fn ended_subobjects_in_constant_evaluation() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A union member ended and written again is a new active member.
    assert_eq!(rule("using I = int; union C { int n; float f; }; constexpr int g() { C c{1}; c.n.~I(); c.n = 3; return c.n; } static_assert(g() == 3);"), None);
    // An element ended and written is outside its life.
    assert_eq!(rule("using I = int; struct R { int v[2]; }; constexpr int g() { R r{{1, 2}}; r.v[1].~I(); r.v[1] = 3; return r.v[0]; } static_assert(g() == 1);"), Some("SA-7"));
    // An ended element leaves the constant without a value.
    assert_eq!(rule("using I = int; struct R { int v[2]; constexpr R() : v{1, 2} { v[1].~I(); } }; constexpr R r;"), Some("CX-1"));
}

#[test]
fn structured_bindings_in_init_statements() {
    assert_eq!(run("struct P { int a, b; }; int main() { int r = 0; if (auto [a, b] = P{1, 2}; a < b) r += b; for (auto [i, n] = P{0, 3}; i < n; ++i) r += 1; switch (auto [a, b] = P{4, 5}; a) { case 4: r += b; } return r; }"), RunOutcome::Result(10));
}

#[test]
fn structured_bindings_of_bit_fields() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A write through the binding keeps the width of the bit-field.
    assert_eq!(run("struct F { unsigned bits : 3; int o; }; int main() { F f{1, 2}; auto& [b, o] = f; b = 9; return f.bits * 10 + b; }"), RunOutcome::Result(11));
    assert_eq!(rule("struct F { int bits : 3; int o; }; void g(F& f) { auto& [b, o] = f; int* p = &b; }"), Some("SB-16"));
}

#[test]
fn captures_of_this() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // `this` refers to the object; `*this` copies it; a default captures
    // `this` at its first use, also through a nested lambda.
    assert_eq!(run("struct C { int n = 0; auto add() { return [this] { ++n; }; } auto copy() { return [*this]() mutable { return ++n; }; } int nested() { return [&] { return [&] { return n + 10; }(); }(); } }; int main() { C c; c.add()(); auto k = c.copy(); int i = k(); return c.n * 100 + i * 10 + c.nested() - 11; }"), RunOutcome::Result(120));
    assert_eq!(rule("struct S { int v; int f() { return [] { return v; }(); } };"), Some("LAM-17"));
    assert_eq!(rule("void f() { auto g = [this] { return 1; }; }"), Some("LAM-24"));
    let o = crate::Options { werror: true, wflags: vec!["-Wextra".into()], ..Default::default() };
    let warned = match crate::check_source("struct S { int v = 0; int f() { return [=] { return v; }(); } };", Path::new("t.cpp"), &o).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(warned, Some("LAM-21"));
}

#[test]
fn default_member_initializers_that_use_this() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // `this` is the object, for a variable, a member of another class, a
    // lambda that captures it, and a constructor that leaves the member out.
    assert_eq!(run("struct S { S* self = this; }; struct T { S s; int v = 5; int got = [this] { return v; }(); T() {} }; int main() { S a; T t; return (a.self == &a) + (t.s.self == &t.s) + t.got; }"), RunOutcome::Result(7));
    assert_eq!(rule("struct C { int base = 6; constexpr int start() const { return base + 1; } int value = start(); }; static_assert(C{}.value == 7); constexpr C c{}; static_assert(c.value == 7);"), None);
}

#[test]
fn parenthesized_aggregate_initialization() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // Each value initializes one whole member, and the rest take their
    // defaults.
    assert_eq!(rule("struct I { int a, b; }; struct O { I i; int c; int d = 7; }; constexpr O o(I{1, 2}, 3); static_assert(o.i.b == 2 && o.c == 3 && o.d == 7); constexpr I one(5); static_assert(one.a == 5 && one.b == 0);"), None);
    assert_eq!(rule("struct I { int a, b; }; struct O { I i; int c; }; O o(1, 2);"), Some("LIST-9"));
    // A temporary that a reference member binds ends with the full
    // expression.
    assert_eq!(rule("struct H { const int& n; }; constexpr int f() { H h(1); return h.n; } static_assert(f() == 1);"), Some("SA-7"));
}

#[test]
fn calls_through_the_address_of_a_member_set() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct I { int reset() { return 1; } static int reset(int v) { return v; } int use() { return (&I::reset)(7); } }; int main() { I i; return i.use(); }"), RunOutcome::Result(7));
    assert_eq!(rule("struct I { void reset(); static void reset(int); void use() { (&I::reset)(); } };"), Some("OVL-21"));
    assert_eq!(rule("struct I { int f() &; static int f(int = 0); int use() { return (&(I::f))(1); } };"), Some("OVL-44"));
    assert_eq!(rule("struct S { ~S() {} }; auto p = &S::~S;"), Some("DTOR-8"));
}

#[test]
fn qualified_friends() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("class C; struct R { int read(const C& c) const; }; class C { int s = 9; friend int R::read(const C&) const; }; int R::read(const C& c) const { return c.s; } int main() { return R{}.read(C{}); }"), RunOutcome::Result(9));
    assert_eq!(rule("class C; struct R { int read(const C&) const; int other(const C&) const; }; class C { int s = 9; friend int R::read(const C&) const; }; int R::other(const C& c) const { return c.s; }"), Some("ACC-2"));
    assert_eq!(rule("class O { void h(); }; class C { friend void O::h(); };"), Some("ACC-15"));
}

#[test]
fn reference_members_whose_default_binds_a_temporary() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // An aggregate list keeps the temporary; a constructor that leaves
    // the member to its default is an error.
    assert_eq!(run("struct A { int tag; const int& r = 5; }; int main() { A a{1}; return a.tag + a.r; }"), RunOutcome::Result(6));
    assert_eq!(run("int v = 3; struct S { const int& r = 5; S() : r(v) {} }; int main() { S s; return s.r; }"), RunOutcome::Result(3));
    assert_eq!(rule("struct S { const int& r = 42; }; S s;"), Some("CTOR-17"));
    assert_eq!(rule("struct S { const int& r = 42; S() {} };"), Some("CTOR-17"));
}

#[test]
fn class_from_another_class() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // The constructor that binds the object without a qualification wins.
    assert_eq!(rule("struct S; struct T { int from = 0; constexpr T(S&) : from(1) {} constexpr T(int f) : from(f) {} }; struct S { constexpr operator T() const { return T(2); } }; constexpr int p() { S s; T t = s; return t.from; } static_assert(p() == 1);"), None);
    // A tie on the object's sequence goes to the candidate with no
    // conversion after it.
    assert_eq!(rule("struct S; struct T { int from; constexpr T(const S&) : from(1) {} constexpr T(int v) : from(v) {} }; struct D : T { constexpr D() : T(2) {} }; struct S { constexpr operator D() const { return D(); } }; constexpr S s; constexpr T t = s; static_assert(t.from == 1);"), None);
    assert_eq!(rule("struct S; struct T { T(const S&); }; struct S { operator T() const; }; S s; T t = s;"), Some("OVL-36"));
    // Direct-initialization counts an explicit function that gives the
    // class itself, not one that gives a derived class.
    assert_eq!(rule("struct T { int v = 0; constexpr T(int x) : v(x) {} }; struct S { constexpr explicit operator T() const { return T(4); } }; constexpr int p() { S s; T t(s); return t.v; } static_assert(p() == 4);"), None);
    assert_eq!(rule("struct B { B() = default; B(const B&) = default; }; struct D : B {}; struct S { explicit operator D() const { return D{}; } }; B b(S{});"), Some("OVL-35"));
}

#[test]
fn braced_constructor_arguments() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct P { int x, y; P(int a, int b) : x(a), y(b) {} }; struct S { int v; S(P p, int k) : v(p.x + p.y + k) {} }; int main() { S s{{1, 2}, 3}; S t({4, 5}, 6); return s.v + t.v; }"), RunOutcome::Result(21));
    assert_eq!(rule("struct S { S(int); }; S f({1}); S g{{1}};"), None);
    assert_eq!(rule("struct S { S(int); }; S s{{1, 2}};"), Some("CTOR-25"));
    assert_eq!(rule("struct S { int x; S(int v) : x(v) {} }; S s{.x = 1};"), Some("ILC-7"));
}

#[test]
fn constructor_declaration_forms() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    for src in ["struct S { S() const; };", "struct S { S() &; };", "struct S { static S(); };", "struct S { S() volatile; };", "struct S { int S(); };"] {
        assert_eq!(rule(src), Some("CTOR-1"), "{}", src);
    }
}

#[test]
fn consteval_and_nodiscard_constructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let src = "struct Id { const char* t; consteval Id(const char* s) : t(s) { if (s[0] >= '0' && s[0] <= '9') throw 0; } }; int len(Id id) { int n = 0; while (id.t[n]) ++n; return n; }";
    assert_eq!(rule(&format!("{} Id a = \"abc\"; int u() {{ return len(\"total\"); }}", src)), None);
    assert_eq!(rule(&format!("{} Id b = \"2x\";", src)), Some("CEV-1"));
    let o = crate::Options { werror: true, ..Default::default() };
    let warned = match crate::check_source("struct S { [[nodiscard]] S(int) {} }; void f() { S(1); }", Path::new("t.cpp"), &o).0 {
        Outcome::Reject(ds) => ds[0].rule,
        _ => None,
    };
    assert_eq!(warned, Some("ATTR-5"));
}

#[test]
fn constexpr_unions_and_volatile_members() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("union U { int a; float b; constexpr U() {} }; constexpr U u;"), None);
    assert_eq!(rule("union R { volatile int h; int s; constexpr R() : s(0) {} }; constexpr R r; static_assert(r.s == 0);"), None);
    assert_eq!(rule("union R { volatile int h; volatile long w; constexpr R() : h(0) {} }; constexpr R r;"), Some("CX-1"));
    assert_eq!(rule("struct P { volatile int l; constexpr P() : l(0) {} }; constexpr P p;"), Some("CX-1"));
}

#[test]
fn user_defined_sequences_in_overload_resolution() {
    // A standard sequence beats a converting constructor, and a converting
    // constructor makes the only viable candidate.
    assert_eq!(run("struct E { int id; E(int i) : id(i) {} }; int f(E e) { return e.id; } int f(long) { return 50; } int g(E e) { return e.id; } int g(const char*) { return 0; } int main() { return f(1L) + g(7); }"), RunOutcome::Result(57));
    // A using-declaration does not hide the overrider of the class itself.
    assert_eq!(run("struct N; struct V { virtual int visit(const N&) = 0; virtual int visit(int) = 0; }; struct N {}; struct W : V { int visit(const N&) override { return 1; } int visit(int) override { return 2; } }; struct C final : W { using W::visit; int visit(const N&) override { return 3; } }; int main() { C c; V& v = c; N n; return v.visit(n) * 10 + v.visit(0); }"), RunOutcome::Result(32));
}

#[test]
fn thread_local_objects_with_constructors() {
    // A use of a constant thread_local starts nothing; the first use of a
    // waiting one starts all of them; a block thread_local starts when
    // its declaration is first reached.
    assert_eq!(run("int made = 0; struct L { L() { ++made; } }; struct C { int t; C() : t(5) { ++made; } }; thread_local L l; thread_local int k = 3; thread_local C c; int main() { int a = k; int b = made; int t = c.t; return a * 100 + b * 10 + made + t; }"), RunOutcome::Result(307));
    assert_eq!(run("int made = 0; struct K { int h = 0; K() { ++made; } }; int look() { thread_local K k; return ++k.h; } int main() { int before = made; look(); return before * 10 + look() + made; }"), RunOutcome::Result(3));
}

#[test]
fn references_bound_by_conversion_functions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct I { int v; }; I shared{1}; struct S { constexpr operator I&&() const { return static_cast<I&&>(shared); } }; constexpr bool f() { const I& r = S{}; return &r == &shared; } static_assert(f());"), None);
    assert_eq!(rule("using F = int(); constexpr int a() { return 1; } struct S { constexpr operator F&&() const { return a; } }; constexpr F& r = S{}; static_assert(r() == 1);"), None);
}

#[test]
fn redefinitions_and_ambiguous_integral_conversions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct S { int a; }; struct S { int a; };"), Some("TYN-7"));
    assert_eq!(rule("struct D {}; struct D final {};"), Some("DER-1"));
    let two = "struct S { operator int() const { return 1; } operator long() const { return 2; } };";
    assert_eq!(rule(&format!("{} void f(S s) {{ switch (s) {{ case 1: break; }} }}", two)), Some("SW-2"));
    assert_eq!(rule(&format!("{} int* f(S s) {{ return new int[s]; }}", two)), Some("CVF-8"));
    assert_eq!(run("struct S { operator int() const { return 2; } }; int main() { S s; int* p = new int[s]; p[1] = 4; int r = p[1]; delete[] p; switch (s) { case 2: return r; } return 0; }"), RunOutcome::Result(4));
}

#[test]
fn decltype_in_a_functional_cast() {
    assert_eq!(run("int main() { auto n = decltype(sizeof 0)(-1) / 2; long k = decltype(3L)(7); return n > 1000 ? int(k) : 0; }"), RunOutcome::Result(7));
}

#[test]
fn delete_of_a_class_operand() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct B { double* d; operator double*() const { return d; } }; int main() { const B b{new double[2]{1.0, 2.0}}; int r = int(b.d[1]); delete[] b; return r; }"), RunOutcome::Result(2));
    assert_eq!(rule("struct S {}; void f(S s) { delete s; }"), Some("NEW-8"));
    assert_eq!(rule("struct S { operator int*(); operator long*(); }; void f(S s) { delete s; }"), Some("NEW-15"));
}

#[test]
fn default_arguments_that_make_special_constructors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct C { int v; C(int x); }; C::C(int x = 5) : v(x) {}"), Some("DEFARG-12"));
    assert_eq!(rule("struct C { C(); C(const C&, int); }; C::C(const C&, int = 0) {}"), Some("DEFARG-12"));
}

#[test]
fn braced_lists_assigned_to_class_objects() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct V { int x, y; V(int a, int b) : x(a), y(b) {} V& operator+=(V o) { x += o.x; y += o.y; return *this; } }; struct P { int a, b; }; int main() { V v{1, 1}; v += {2, 3}; P p{0, 0}; p = {4, 5}; return v.x * 1000 + v.y * 100 + p.a * 10 + p.b; }"), RunOutcome::Result(3445));
    // The implicit assignment takes a class prvalue on the left.
    assert_eq!(rule("struct P { int a, b; }; P make(); void f() { make() = {1, 2}; make() = P{3, 4}; }"), None);
}

#[test]
fn braced_arguments_among_overloads() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("constexpr int n(const int (&)[2]) { return 2; } constexpr int n(const int (&)[3]) { return 3; } static_assert(n({1, 2, 3}) == 3);"), None);
    assert_eq!(rule("struct Q { constexpr operator long() const { return 2; } }; constexpr int f(const int (&)[2]) { return 1; } constexpr int f(const long (&)[2]) { return 2; } static_assert(f({Q{}, 0}) == 2);"), None);
    assert_eq!(rule("struct L { L(int); }; struct I {}; int show(L); long show(...); void u(I i) { show({i}); }"), Some("OVL-35"));
}

#[test]
fn destructors_defined_outside_the_class() {
    assert_eq!(run("int n = 0; struct A { virtual ~A() = 0; }; A::~A() { ++n; } struct B : A {}; struct S { ~S(); }; S::~S() { n += 10; } int main() { { B b; S s; } return n; }"), RunOutcome::Result(11));
}

#[test]
fn calls_with_a_contrived_object() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct S { static int put(long v) { return int(v); } int put(int) && { return 0; } static int fill() { return put(5L); } }; int main() { return S::fill(); }"), RunOutcome::Result(5));
    assert_eq!(rule("struct S { static void put(long); void put(int) &&; static void fill() { put(1); } };"), Some("OVL-23"));
    assert_eq!(rule("struct I { static void reset(int = 0); void reset(); static void clear() { reset(); } };"), Some("OVL-36"));
}

#[test]
fn this_in_member_declarations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct S { int v; auto get() const -> decltype(this->v) { return v; } };"), None);
    assert_eq!(rule("struct S { int f(const S* p = this); };"), Some("DEFARG-4"));
    assert_eq!(rule("struct B { int size; int fill(unsigned long n = sizeof(*this)); };"), Some("DEFARG-4"));
}

#[test]
fn common_initial_sequence_in_constant_evaluation() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].cause.or(ds[0].rule),
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct A { int t; int a; }; struct B { int t; double b; }; union U { A a; B b; }; constexpr int f() { U u{.a = {7, 1}}; return u.b.t; } static_assert(f() == 7);"), Some("UNI-12"));
}

#[test]
fn references_from_braces_and_explicit_conversions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // A braced class object makes a temporary of the referenced type.
    assert_eq!(run("int t = 1; struct S { operator int&() { return t; } }; S s; int main() { const int& a{s}; t = 7; return a; }"), RunOutcome::Result(1));
    assert_eq!(rule("int t = 1; struct S { operator int&() { return t; } }; S s; int& a{s};"), Some("REF-9"));
    // Direct-initialization takes an explicit conversion function, and
    // copy-initialization does not.
    assert_eq!(run("int v = 5; struct S { explicit operator int&() const { return v; } }; S s; const int& r(s); int main() { return &r == &v ? 3 : 0; }"), RunOutcome::Result(3));
    assert_eq!(rule("int v = 5; struct S { explicit operator int&() const { return v; } }; S s; const int& r = s;"), Some("OVL-35"));
}

#[test]
fn functions_that_hide_a_class() {
    assert_eq!(run("struct stat { int size = 8; }; int stat(int x) { return x; } int main() { struct stat s; return s.size + stat(3); }"), RunOutcome::Result(11));
}

#[test]
fn new_value_initializes_a_class() {
    assert_eq!(run("struct P { int a; int b = 4; }; int main() { P* p = new P(); int r = p->a * 10 + p->b; delete p; return r; }"), RunOutcome::Result(4));
}

#[test]
fn members_of_qualified_function_types() {
    assert_eq!(run("using G = int() &&; using F = int() const; struct S { G take; F get; int v = 4; }; int S::take() && { return v; } int S::get() const { return v + 1; } G S::* pm = &S::take; F S::* pg = &S::get; int main() { S s; return (S{}.*pm)() * 10 + (s.*pg)(); }"), RunOutcome::Result(45));
}

#[test]
fn friend_types_and_member_specifier_errors() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct R; using A = R; class C { int s = 2; friend A; friend int; }; struct R { int get(const C& c) { return c.s; } }; int main() { return R{}.get(C{}); }"), RunOutcome::Result(2));
    assert_eq!(rule("struct S { friend void f() = 0; };"), Some("VIRT-9"));
    assert_eq!(rule("struct S { inline int v = 0; };"), Some("INL-2"));
    assert_eq!(rule("struct S { void f() = delete(\"use g\"); };"), Some("DEL-3"));
}

#[test]
fn enumerations_in_classes_and_qualified_definitions() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct S { enum class K : short; }; enum class S::K : short { low = 1, high = 9 }; constexpr S::K k = S::K::high; static_assert(static_cast<int>(k) == 9);"), None);
    assert_eq!(rule("namespace lib { constexpr int start = 4; enum Plain : int; } enum lib::Plain : int { first = start + 1 }; static_assert(lib::first == 5);"), None);
    assert_eq!(run("namespace p { enum class C { r, g, b }; } struct T { using enum p::C; }; int main() { return static_cast<int>(T::b); }"), RunOutcome::Result(2));
}

#[test]
fn bindings_captured_and_destructor_names() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("struct P { int a; int b; }; int main() { auto [x, y] = P{1, 2}; auto g = [x, &y] { return x + y; }; y = 5; return g(); }"), RunOutcome::Result(6));
    assert_eq!(rule("struct B { ~B() {} }; struct D : B { int c = 0; }; void e(D& d) { d.B::~B(); } void g(D* d) { d->B::~B(); }"), None);
    assert_eq!(rule("struct S { ~S() {} }; void f(S& s) { s.~S; }"), Some("DTOR-8"));
    assert_eq!(rule("struct R { int a; int& r; }; R x{1};"), Some("LIST-12"));
}

#[test]
fn operator_functions_for_enumeration_operands() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("enum C { red, green }; int calls = 0; C operator~(C c) { ++calls; return c == red ? green : red; } int main() { C c = ~red; return calls * 10 + c; }"), RunOutcome::Result(11));
    // A non-member function without a parameter of the enumeration is no
    // candidate, so the built-in operator applies.
    assert!(rule("enum class M { on }; struct W { W(M); }; bool operator+(W, W); auto r = M::on + M::on;").is_some());
}

#[test]
fn member_calls_in_lambda_bodies() {
    assert_eq!(run("struct S { int v = 3; int get() { return v; } int f() { return [&] { return get() + [this] { return get(); }(); }(); } }; int main() { S s; return s.f(); }"), RunOutcome::Result(6));
}

#[test]
fn member_type_access_outside_function_bodies() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("class O { struct H { int v; }; static H make(); }; O::H O::make() { return H{1}; }"), None);
    assert_eq!(rule("class O { struct H { int v; }; public: static H make(); }; O::H h;"), Some("ACC-2"));
    assert_eq!(rule("class B { protected: using K = long; }; class D : public B { public: using B::K; }; D::K k = 1;"), None);
}

#[test]
fn later_types_in_defaults_and_ambiguous_injected_names() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("struct G { int bytes(int n = sizeof(Cell)) { return n; } struct Cell { char d[5]; }; }; static_assert(sizeof(G::Cell) == 5);"), None);
    assert_eq!(rule("namespace a { struct base {}; } namespace b { struct base {}; } struct j : a::base, b::base { void f() { base* p = nullptr; (void)p; } };"), Some("DER-4"));
}

#[test]
fn function_templates_with_type_parameters() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    // Deduction by value, by reference, through a pointer, and with an
    // explicit argument; a recursive specialization finds itself.
    assert_eq!(run("template <class T> T twice(T v) { return v + v; } template <class T> void put(T& r, const T& v) { r = v; } template <class T> T get(T* p) { return *p; } template <class T> T down(T n) { return n == 0 ? 0 : 1 + down(n - 1); } int main() { int x = 0; put(x, 4); long l = 2; return twice(3) + twice<long>(1) + x + get(&l) + int(down(3L)); }"), RunOutcome::Result(17));
    assert_eq!(rule("template <class T> constexpr T sq(T v) { return v * v; } static_assert(sq(3) == 9 && sq(2.0) == 4.0);"), None);
    // Two deductions that differ give no candidate.
    assert_eq!(rule("template <class T> T pick(T a, T b) { return a; } int f() { return pick(1, 2L); }"), Some("unsupported"));
    // A template that nothing specializes is checked with `int` for its
    // parameters; one that this trial does not check stays unsupported.
    assert_eq!(rule("template <class T> T id(T v) { return v; }"), None);
    assert_eq!(rule("template <class T> T id(T v) { return v.x; }"), Some("unsupported"));
}

#[test]
fn variable_templates_with_partial_specializations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class T, class U> constexpr bool same = false; template <class T> constexpr bool same<T, T> = true;";
    assert_eq!(rule(&format!("{} static_assert(same<int, int> && !same<int, long> && same<const int*, const int*>);", same)), None);
    assert_eq!(rule(&format!("{} static_assert(same<int, long>);", same)), Some("SA-1"));
    assert_eq!(rule("template <class T> constexpr int size = sizeof(T); template <class T> constexpr int size<T*> = 1; static_assert(size<long> == 8 && size<char*> == 1);"), None);
}

#[test]
fn reference_collapsing() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("template <class A, class B> constexpr bool same = false; template <class A> constexpr bool same<A, A> = true; using L = int&; using R = int&&; static_assert(same<L&, int&> && same<L&&, int&> && same<R&, int&> && same<R&&, int&&>); int x; L& a = x; R&& b = 1;"), None);
    assert_eq!(rule("int x; int& & r = x;"), Some("DCL-18"));
}

#[test]
fn class_templates_with_type_parameters() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("template <class T> struct Box { T v; T get() const { return v; } Box<T> self() const { return *this; } }; template <class A, class B> struct Pair { A a; B b; }; int main() { Box<int> b{3}; Pair<Box<int>, long> p{b, 4}; return b.self().get() + p.a.v + int(p.b); }"), RunOutcome::Result(10));
    assert_eq!(rule("template <class T> struct Box { T v; }; static_assert(sizeof(Box<char>) == 1 && sizeof(Box<long>) == 8);"), None);
    // A class template that nothing specializes is checked with `int`
    // for its parameters; a definition that this trial does not check
    // leaves the unit unsupported.
    assert_eq!(rule("template <class T> struct Box { T v; };"), None);
    assert_eq!(rule("template <class T> struct Box { static_assert(sizeof(T) == 1); };"), Some("unsupported"));
}

#[test]
fn union_and_qualified_class_templates() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("template <class T> union W { T x; char c; }; static_assert(sizeof(W<int>) == 4); namespace lib { template <class E> class Box { decltype(sizeof 0) n_; public: constexpr Box() : n_(0) {} }; } lib::Box<int> b;"), None);
    // The namespaces of a specialization's arguments are associated.
    assert_eq!(rule("namespace t { template <class X> struct box {}; } namespace n { struct S {}; int probe(t::box<S>) { return 1; } } int read() { return probe(t::box<n::S>{}); }"), None);
}

#[test]
fn standard_sequence_tie_breakers() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class T, class U> constexpr bool same = false; template <class T> constexpr bool same<T, T> = true; struct One {}; struct Two {};";
    // A pointer to bool loses; a nearer base wins; a base pointer beats
    // void*; a fixed enumeration's own type wins; fewer cv-qualifiers win.
    assert_eq!(rule(&format!("{} int* p; One a(bool); Two a(void*); static_assert(same<decltype(a(p)), Two>); struct B {{}}; struct M : B {{}}; struct D : M {{}}; D* dp; One b(B*); Two b(M*); static_assert(same<decltype(b(dp)), Two>); One c(B*); Two c(void*); static_assert(same<decltype(c(dp)), One>); enum L : short {{ lo }}; One e(short); Two e(int); static_assert(same<decltype(e(lo)), One>); One q(const int*); Two q(const volatile int*); static_assert(same<decltype(q(p)), One>);", same)), None);
}

#[test]
fn more_sequence_tie_breakers() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class T, class U> constexpr bool same = false; template <class T> constexpr bool same<T, T> = true; struct One {}; struct Two {};";
    assert_eq!(rule(&format!("{} void run(); One f(void (&)()); Two f(void (&&)()); static_assert(same<decltype(f(run)), One>); One g(int (&&)[2]); Two g(int (&&)[3]); static_assert(same<decltype(g({{1, 2}})), One>); int (*row)[3]; One h(const int (*)[3]); Two h(const int (*)[]); static_assert(same<decltype(h(row)), One>);", same)), None);
    assert_eq!(rule(&format!("{} struct I {{}}; struct S {{ operator I(); }}; S s; One t(I&&); Two t(const I&); static_assert(same<decltype(t(s)), One>);", same)), None);
}

#[test]
fn conditional_throw_and_class_cv() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("int x = 4; int main() { (true ? x : throw 1) = 5; return x + (false ? throw 1 : 2); }"), RunOutcome::Result(7));
    let same = "template <class T, class U> constexpr bool same = false; template <class T> constexpr bool same<T, T> = true; template <class T> T&& mv(T& v) { return static_cast<T&&>(v); }";
    assert_eq!(rule(&format!("{} struct B {{}}; B b; const B f{{}}; B make(); int i; const int ci = 0; extern bool c; static_assert(same<decltype(c ? f : make()), const B>); static_assert(same<decltype(c ? b : mv(f)), const B>); static_assert(same<decltype(c ? mv(i) : mv(ci)), const int&&>);", same)), None);
}

#[test]
fn decltype_of_names_and_trailing_returns() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class A, class B> constexpr bool same = false; template <class A> constexpr bool same<A, A> = true;";
    assert_eq!(rule(&format!("{} auto add(int a, long b) -> decltype(a + b) {{ return a + b; }} static_assert(same<decltype(add(1, 2)), long>); struct S {{ int m; }}; static_assert(same<decltype(S::m), int>); int g = 0; struct R {{ int& r; }}; R rr{{g}}; auto [x] = rr; static_assert(same<decltype(x), int&>); void f() {{ int y = 0; [=] {{ static_assert(same<decltype(y), int>); static_assert(same<decltype((y)), const int&>); }}(); }}", same)), None);
}

#[test]
fn constant_template_parameters() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("template <class T, int N> struct Arr { T v[N]; constexpr int size() const { return N; } }; template <int N> constexpr int sq = N * N; template <int N, class T> T times(T v) { return v * N; } static_assert(sizeof(Arr<int, 4>) == 16 && sq<5> == 25); int main() { Arr<char, 3> a; return times<3>(2) + a.size(); }"), RunOutcome::Result(9));
    assert_eq!(rule("template <unsigned char N> struct Byte {}; Byte<1> ok; Byte<300> b;"), Some("TARG-13"));
    assert_eq!(rule("template <int N> const int* where() { return &N; } const int* use() { return where<1>(); }"), Some("unsupported"));
}

#[test]
fn explicit_specializations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("template <class T> struct Box { int f() { return 1; } }; template <> struct Box<int> { int f() { return 2; } int extra; }; template <class T> constexpr int code = 0; template <> constexpr int code<char> = 7; int main() { Box<long> a; Box<int> b; return a.f() + b.f() * 10 + code<char> * 100 + code<int> + sizeof(Box<int>) * 1000; }"), RunOutcome::Result(4721));
    assert_eq!(rule("template <class T> struct Box { T v; }; Box<int> early; template <> struct Box<int> { long v; };"), Some("TSP-6"));
    assert_eq!(rule("template <class T> int n = 0; template <> static int n<char> = 1; int r = n<int>;"), Some("TSP-24"));
}

#[test]
fn explicit_function_specializations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("template <class T> constexpr int code(T) { return 1; } template <> constexpr int code<int>(int) { return 2; } template <> constexpr int code(char) { return 3; } static_assert(code(1.0) == 1 && code(1) == 2 && code('a') == 3);"), None);
    assert_eq!(rule("template <class T> int code(T) { return 1; } int a = code(1); template <> int code(int) { return 2; }"), Some("TSP-6"));
}

#[test]
fn template_id_qualifiers_and_depth() {
    let rule = |src: &str, depth: Option<u32>| match crate::check_source(src, Path::new("t.cpp"), &crate::Options { depth, ..Default::default() }).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("namespace q { template <class T> struct Box { static constexpr int n = sizeof(T); using V = T; }; } static_assert(q::Box<int>::n == 4); q::Box<char>::V c; static_assert(sizeof c == 1);", None), None);
    let count = "template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; }; template <> struct Count<0> { static constexpr int value = 0; };";
    assert_eq!(rule(&format!("{} static_assert(Count<50>::value == 50);", count), None), None);
    assert_eq!(rule(&format!("{} static_assert(Count<50>::value == 50);", count), Some(40)), Some("TR-13"));
}

#[test]
fn class_template_members_outside() {
    assert_eq!(run("template <class T> struct Counter { T get() const; static T start; T value = start; }; template <class T> T Counter<T>::get() const { return value; } template <class T> T Counter<T>::start = 7; int main() { return Counter<int>().get() + Counter<long>().get(); }"), RunOutcome::Result(14));
}

#[test]
fn class_partial_specializations_and_typename() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let traits = "template <class T> struct rr { using type = T; }; template <class T> struct rr<T&> { using type = T; }; template <class A, class B> struct same { static constexpr bool value = false; }; template <class A> struct same<A, A> { static constexpr bool value = true; }; template <class T> constexpr typename rr<T>::type&& mv(T&& v) noexcept { return static_cast<typename rr<T>::type&&>(v); }";
    assert_eq!(rule(&format!("{} template <class T> struct size; template <class T> struct size<T*> {{ static constexpr int value = 8; }}; static_assert(same<rr<int&>::type, int>::value && !same<int, long>::value && size<int*>::value == 8); int i; static_assert(same<decltype(mv(i)), int&&>::value);", traits)), None);
    assert_eq!(rule("template <class T, class U> struct Pair {}; template <class T, class U> struct Pair<U, T> {};"), Some("TSP-11"));
    assert_eq!(rule("template <class T> struct Box { int T; };"), Some("TPL-18"));
    assert_eq!(rule("extern \"C\" { template <class T> struct Box {}; }"), Some("TPL-3"));
    // A constructor's parameter owns its argument's object.
    assert_eq!(run("int frees = 0, x; struct B { int* p; explicit B(int* q) : p(q) {} B(B&& o) noexcept : p(o.p) { o.p = nullptr; } ~B() { if (p) ++frees; } }; struct S { B b; S(B v) : b(static_cast<B&&>(v)) {} }; int main() { { B c(&x); S s(static_cast<B&&>(c)); } return frees; }"), RunOutcome::Result(1));
}

#[test]
fn function_template_trials_and_adl() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(rule("template <class T> T twice(T v) { return v + v; }"), None);
    assert_eq!(rule("template <class T> int main() { return 0; }"), Some("TPL-29"));
    assert_eq!(rule("template <class T> void one(T x) { int y{x...}; }"), Some("PACK-8"));
    assert_eq!(rule("namespace n { template <class T> int probe(T) { return 1; } struct S {}; } int r = probe(n::S{});"), None);
    assert_eq!(rule("template <class X, class Y> constexpr bool same = false; template <class X> constexpr bool same<X, X> = true; struct A { operator const int*(); }; struct B { operator void*(); }; A a; B b; extern bool c; static_assert(same<decltype(c ? a : b), const void*>);"), None);
}

#[test]
fn function_template_declarations_and_dependent_specializations() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class A, class B> constexpr bool same = false; template <class A> constexpr bool same<A, A> = true;";
    assert_eq!(rule(&format!("{} template <class T> struct Tag {{}}; template <class T> Tag<T> cref(const T&); template <class T> Tag<T> peel(Tag<T>); const int c = 1; static_assert(same<decltype(cref(c)), Tag<int>>); static_assert(same<decltype(peel(Tag<long>{{}})), Tag<long>>); template <class T> auto add(T a, T b) -> decltype(a + b); static_assert(same<decltype(add('a', 'b')), int>);", same)), None);
    assert_eq!(rule("template <class T> void place(T, int); template <class T> void place(T, int = 1);"), Some("DEFARG-13"));
    assert_eq!(run("template <class T> T twice(T); int main() { return twice(4); } template <class T> T twice(T v) { return v + v; }"), RunOutcome::Result(8));
}

#[test]
fn template_id_bases() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    assert_eq!(run("template <int K, class T> struct Slot { T held; Slot(T v) : held(v) {} }; struct Couple : Slot<0, int>, Slot<1, int> { Couple(int a, int b) : Slot<0, int>(a), Slot<1, int>(b) {} }; int main() { Couple c(5, 9); const Slot<1, int>& s = c; return s.held; }"), RunOutcome::Result(9));
    // A class can be a template argument of its own base (CRTP).
    assert_eq!(run("template <class D> struct Base { int three() { return 3; } }; struct R : Base<R> {}; int main() { R r; return r.three(); }"), RunOutcome::Result(3));
    assert_eq!(rule("template <class T> struct Base {}; struct Both : Base<int>, Base<char> {}; template <class T> void up(Base<T>&); Both b; void use() { up(b); }"), Some("unsupported"));
}

#[test]
fn default_template_arguments() {
    let rule = |src: &str| match crate::check_source(src, Path::new("t.cpp"), &crate::Options::default()).0 {
        Outcome::Reject(ds) => ds[0].rule,
        Outcome::Accept => None,
        Outcome::Unsupported(_) => Some("unsupported"),
    };
    let same = "template <class A, class B> constexpr bool same = false; template <class A> constexpr bool same<A, A> = true;";
    assert_eq!(rule(&format!("{} template <class T = int, class U = T*> struct Pair {{ using second = U; }}; static_assert(same<Pair<>::second, int*> && same<Pair<char>::second, char*>); template <int N, int M = N * 2> constexpr int sum = N + M; static_assert(sum<3> == 9); template <class T = int, class U> constexpr int first(U) {{ return sizeof(T); }} static_assert(first(1.0) == 4);", same)), None);
    assert_eq!(rule("template <class T = int, class U> struct Pair {};"), Some("TPL-14"));
    assert_eq!(rule("template <class T = int> struct Box; template <class T = int> struct Box {};"), Some("TPL-16"));
    assert_eq!(rule("template <class T, class U = int> struct Pair; template <class T = char, class U> struct Pair {}; Pair<> p;"), None);
}
