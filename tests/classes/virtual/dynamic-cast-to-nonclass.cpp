// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: is not a class type
// tidal-error-gcc: target is not pointer or reference to c
// tidal-error-edg: must be a pointer or reference to a complete class type
struct B { virtual ~B() {} };
int* f(B* p) { return dynamic_cast<int*>(p); }
