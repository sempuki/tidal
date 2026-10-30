// tidal-rules: DTOR-6
// tidal-expect: reject
using L = long;
void f(int i) { i.~L(); }
