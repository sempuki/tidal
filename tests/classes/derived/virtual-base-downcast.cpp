// tidal-rules: DER-6
// tidal-expect: reject
struct A {};
struct B : virtual A {};
B* f(A* a) { return static_cast<B*>(a); }
