// tidal-rules: DER-11
// tidal-expect: reject
// tidal-error-clang: via virtual base
// tidal-error-gcc: because the base is virtual
// tidal-error-edg: base class is virtual
struct A {};
struct B : virtual A {};
B* f(A* a) { return static_cast<B*>(a); }
