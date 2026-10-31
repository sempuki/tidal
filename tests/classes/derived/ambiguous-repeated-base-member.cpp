// tidal-rules: DER-4
// tidal-expect: reject
struct A { int v; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int f(D& d) { return d.v; }
