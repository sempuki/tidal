// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: read-only variable is not assignable
// tidal-error-gcc: assignment of read-only location
// tidal-error-edg: must be a modifiable lvalue
// tidal-note: Gap review 2026-10-31.
struct S { mutable int m = 0; };
void f(const S& s) { int S::* pm = &S::m; s.*pm = 1; }
