// tidal-rules: ACC-8
// tidal-expect: reject
struct B { int x; };
class D : B {};
int f(D& d) { return d.x; }
