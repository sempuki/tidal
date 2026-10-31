// tidal-rules: DER-4
// tidal-expect: reject
struct A { int x; };
struct B : A { int x; };
struct C : A {};
struct D : B, C {};
int f(D& d) { return d.x; }
