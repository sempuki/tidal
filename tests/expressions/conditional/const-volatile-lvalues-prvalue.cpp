// tidal-rules: COND-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
const int c = 1;
volatile int v = 2;
auto p = &(true ? c : v);
