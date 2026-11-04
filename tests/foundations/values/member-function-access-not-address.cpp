// tidal-rules: VC-12
// tidal-expect: reject
// tidal-error-clang: non-constant pointer to member function
// tidal-error-gcc: address of a bound member function
// tidal-error-edg: pointer to a bound function
// tidal-note: Gap review 2026-10-31 (third).
struct S { int g(); };
auto f(S s) { return &s.g; }
