// tidal-rules: DER-6
// tidal-expect: accept
struct A {};
struct B : virtual A {};
struct C : virtual A {};
struct D : B, C {};
A* f(D* d) { return d; }
