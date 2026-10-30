// tidal-rules: CPY-11
// tidal-expect: reject
int& f() { int x = 0; return x; }
int& g(int&& r) { return r; }
