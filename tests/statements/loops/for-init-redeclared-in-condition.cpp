// tidal-rules: LOOP-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { for (int i = 0; int i = 1;) break; }
