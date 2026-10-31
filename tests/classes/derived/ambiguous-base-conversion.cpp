// tidal-rules: DER-5
// tidal-expect: reject
struct A {};
struct B : A {};
struct C : A {};
struct D : B, C {};
A* f(D* d) { return d; }
