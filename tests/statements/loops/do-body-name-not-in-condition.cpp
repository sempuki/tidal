// tidal-rules: LOOP-7
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .done
// tidal-error-gcc: done. was not declared in this scope
// tidal-error-edg: identifier .done. is undefined
// tidal-note: Gap review 2026-10-31 (third).
void f() { do { int done = 1; } while (!done); }
