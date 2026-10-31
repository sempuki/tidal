// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: case. statement not in switch statement
// tidal-error-gcc: not within a switch statement
// tidal-error-edg: case label may only be used within a switch
// tidal-note: Gap review 2026-10-31 (third).
void f(int v) { switch (v) { case 0: { struct L { void g() { case 1: ; } }; } } }
