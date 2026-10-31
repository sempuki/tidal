// tidal-rules: SW-6
// tidal-expect: reject
// tidal-error-clang: redefinition of .a
// tidal-error-gcc: redeclaration of .int a
// tidal-error-edg: declared in condition, may not be redeclared
// tidal-note: Gap review 2026-10-31 (third).
void f() { switch (int a = 1; a) { int a = 2; } }
