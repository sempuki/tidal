// tidal-rules: DER-5
// tidal-expect: accept
struct A {};
struct B : A {};
struct C : A {};
struct D : B, C {};
A* f(D* d) { return static_cast<B*>(d); }
