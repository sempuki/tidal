// tidal-rules: SW-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { switch (int a = 1; a) { int a = 2; } }
