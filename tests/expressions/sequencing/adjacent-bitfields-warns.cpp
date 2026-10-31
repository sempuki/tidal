// tidal-rules: SEQ-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct F { unsigned a : 4; unsigned b : 4; };
int f(F& x) { return (x.a = 1) + (x.b = 2); }
