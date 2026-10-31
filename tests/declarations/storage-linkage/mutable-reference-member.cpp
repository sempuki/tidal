// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: mutable. cannot be applied to references
// tidal-error-gcc: reference .r. cannot be declared .mutable
// tidal-error-edg: reference with .mutable. is nonstandard
// tidal-note: Gap review 2026-10-31.
int g;
struct S { mutable int& r = g; };
