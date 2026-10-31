// tidal-rules: DER-6
// tidal-expect: reject
struct A {};
struct B : virtual A {};
struct C : A {};
struct D : B, C {};
A* f(D* d) { return d; }
