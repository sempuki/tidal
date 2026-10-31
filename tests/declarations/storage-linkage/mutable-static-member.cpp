// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .mutable. declaration specifier
// tidal-error-gcc: static. specifier conflicts with .mutable
// tidal-error-edg: more than one storage class
// tidal-note: Gap review 2026-10-31.
struct S { mutable static int n; };
