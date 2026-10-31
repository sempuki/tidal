// tidal-rules: DER-3
// tidal-expect: reject
struct B { int f(); };
struct D : B { int f = 0; };
int g(D& d) { return d.f(); }
