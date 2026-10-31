// tidal-rules: COMMA-8
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
int f(int v) { switch (v) { case 1, 2: return 1; } return 0; }
