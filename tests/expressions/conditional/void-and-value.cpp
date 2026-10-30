// tidal-rules: COND-4
// tidal-expect: reject
void f();
int g(bool c) { return c ? f() : 1; }
