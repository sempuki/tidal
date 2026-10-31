// tidal-rules: LOOP-7
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { do { int done = 1; } while (!done); }
