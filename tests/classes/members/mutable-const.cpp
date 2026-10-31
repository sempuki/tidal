// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: mutable. and .const. cannot be mixed
// tidal-error-gcc: cannot be declared .mutable
// tidal-error-edg: mutable is not allowed
// tidal-note: Gap review 2026-10-30.
struct S { mutable const int c = 1; };
