// tidal-rules: COND-10
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int f(bool c, int a, int b) { return c ? a++, b : a; }
