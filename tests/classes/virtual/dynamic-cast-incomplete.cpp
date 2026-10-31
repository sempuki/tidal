// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: is an incomplete type
// tidal-error-gcc: target is not pointer or reference
// tidal-error-edg: must be a pointer or reference to a complete class type
struct B { virtual ~B() {} };
struct D;
D* f(B* p) { return dynamic_cast<D*>(p); }
