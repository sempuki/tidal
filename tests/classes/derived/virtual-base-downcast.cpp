// tidal-rules: DER-11
// tidal-expect: reject
struct A {};
struct B : virtual A {};
B* f(A* a) { return static_cast<B*>(a); }
