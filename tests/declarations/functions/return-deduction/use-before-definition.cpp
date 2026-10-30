// tidal-rules: RET-3
// tidal-expect: reject
auto f();
int x = f();
auto f() { return 1; }
