// tidal-rules: LOOP-7
// tidal-expect: reject
// tidal-error-clang: redefinition of .i
// tidal-error-gcc: redeclaration of .int i
// tidal-error-edg: declared in for-loop initialization, may not be redeclared
// tidal-note: Gap review 2026-10-31 (third).
void f() { for (int i = 0; int i = 1;) break; }
