// tidal-rules: ACC-8
// tidal-expect: accept
struct B { int x; };
struct D : B {};
int f(D& d) { return d.x; }
