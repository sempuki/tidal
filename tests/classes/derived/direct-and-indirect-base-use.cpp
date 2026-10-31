// tidal-rules: DER-18
// tidal-expect: reject
struct A { int v; };
struct B : A {};
struct D : A, B {};
int f(D& d) { return d.v; }
