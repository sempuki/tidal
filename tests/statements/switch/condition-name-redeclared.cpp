// tidal-rules: SW-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { switch (int c = 1) { case 1: { } default: break; int c = 2; } }
