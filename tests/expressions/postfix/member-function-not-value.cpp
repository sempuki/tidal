// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-error-clang: reference to non-static member function must be called
// tidal-error-gcc: cannot convert .S::f. from type
// tidal-error-edg: pointer to a bound function may only be used to call
// tidal-note: Gap review 2026-10-31.
struct S { int f() { return 1; } };
void g(S s) { auto x = s.f; }
