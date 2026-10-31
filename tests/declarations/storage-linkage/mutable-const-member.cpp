// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: mutable. and .const. cannot be mixed
// tidal-error-gcc: const. .c. cannot be declared .mutable
// tidal-error-edg: mutable is not allowed
// tidal-note: Gap review 2026-10-31.
struct S { mutable const int c = 0; };
