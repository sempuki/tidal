// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: mutable. can only be applied to member variables
// tidal-error-gcc: non-member .x. cannot be declared .mutable
// tidal-error-edg: mutable is not allowed
// tidal-note: Gap review 2026-10-31.
void f() { mutable int x = 0; }
