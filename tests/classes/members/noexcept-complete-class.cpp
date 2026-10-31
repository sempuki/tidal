// tidal-rules: CLS-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { void f() noexcept(noexcept(g())); void g() noexcept; };
