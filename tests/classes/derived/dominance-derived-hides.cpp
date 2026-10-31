// tidal-rules: DER-4
// tidal-expect: accept
struct A { int x; };
struct B : virtual A { int x; };
struct C : virtual A {};
struct D : B, C {};
int f(D& d) { return d.x; }
