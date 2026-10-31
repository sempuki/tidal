// tidal-rules: COND-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
const int c = 1;
int m = 2;
const int* p = &(true ? c : m);
