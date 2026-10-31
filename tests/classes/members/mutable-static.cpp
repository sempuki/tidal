// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .mutable
// tidal-error-gcc: static. specifier conflicts with .mutable
// tidal-error-edg: more than one storage class
// tidal-note: Gap review 2026-10-30.
struct S { mutable static int s; };
