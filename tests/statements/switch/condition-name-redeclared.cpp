// tidal-rules: SW-4
// tidal-expect: reject
// tidal-error-clang: redefinition of .c
// tidal-error-gcc: redeclaration of .int c
// tidal-error-edg: declared in condition, may not be redeclared
// tidal-note: Gap review 2026-10-31 (third).
void f() { switch (int c = 1) { case 1: { } default: break; int c = 2; } }
