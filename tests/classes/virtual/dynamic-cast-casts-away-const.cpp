// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: casts away qualifiers
// tidal-error-gcc: casts away constness
// tidal-error-edg: cannot cast away const
struct B { virtual ~B() {} };
struct D : B {};
D* f(const B* p) { return dynamic_cast<D*>(p); }
