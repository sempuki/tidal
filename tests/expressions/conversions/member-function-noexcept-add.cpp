// tidal-rules: CONV-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S { void f(); };
void (S::* p)() noexcept = &S::f;
