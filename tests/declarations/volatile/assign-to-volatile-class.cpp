// tidal-rules: VOL-4
// tidal-expect: reject
struct S { int v; };
void f(volatile S& d, const S& s) { d = s; }
