// tidal-rules: PRIM-12
// tidal-expect: reject
// tidal-error-clang: within const member function
// tidal-error-gcc: in read-only object
// tidal-error-edg: must be a modifiable lvalue
// tidal-note: Gap review 2026-10-31.
struct S { int v = 0; void set() const { v = 1; } };
