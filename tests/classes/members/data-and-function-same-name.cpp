// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: redefinition of .f. as different kind of symbol
// tidal-error-gcc: conflicts with a previous declaration
// tidal-error-edg: has already been declared in the current scope
// tidal-note: Gap review 2026-10-31.
struct S { int f; void f(); };
