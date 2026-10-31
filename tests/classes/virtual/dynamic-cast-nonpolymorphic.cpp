// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: is not polymorphic
// tidal-error-gcc: source type is not polymorphic
// tidal-error-edg: must have a polymorphic class type
struct B {};
struct D : B {};
D* f(B* p) { return dynamic_cast<D*>(p); }
