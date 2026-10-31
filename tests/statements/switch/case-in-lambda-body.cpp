// tidal-rules: SW-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f(int v) { switch (v) { case 0: { auto l = [] { case 1: return; }; l(); } } }
