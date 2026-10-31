// tidal-rules: DER-4
// tidal-expect: reject
struct A { int x; };
struct B { int x; };
struct D : A, B {};
int f(D& d) { return d.x; }
