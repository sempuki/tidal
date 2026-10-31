// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: mutable. cannot be applied to functions
// tidal-error-gcc: function .f. cannot be declared .mutable
// tidal-error-edg: mutable is not allowed
// tidal-note: Gap review 2026-10-31.
struct S { mutable int f(); };
