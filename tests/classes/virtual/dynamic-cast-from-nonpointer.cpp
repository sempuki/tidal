// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: cannot use dynamic_cast to convert from .B. to .D \*
// tidal-error-gcc: source is not a pointer
// tidal-error-edg: operand of a pointer dynamic_cast must be a pointer
struct B { virtual ~B() {} };
struct D : B {};
D* f(B b) { return dynamic_cast<D*>(b); }
