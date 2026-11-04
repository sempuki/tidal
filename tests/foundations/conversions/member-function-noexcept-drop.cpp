// tidal-rules: CONV-10
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct S { void f() noexcept; };
void (S::* p)() = &S::f;
