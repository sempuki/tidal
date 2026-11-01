// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not allowed in a converted constant expression
// tidal-error-gcc: in a converted constant expression
// tidal-error-edg: is incompatible with template parameter
struct Base { int b; };
struct Derived : Base { int d; };
Derived object;
template <Base* P> struct At {};
At<&object> a;
