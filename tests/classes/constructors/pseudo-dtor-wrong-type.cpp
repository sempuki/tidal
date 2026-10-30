// tidal-rules: DTOR-8
// tidal-expect: reject
using L = long;
void f(int i) { i.~L(); }
