// tidal-rules: COND-5
// tidal-expect: reject
void f(bool c, int* p) { auto r = c ? p : 1; }
