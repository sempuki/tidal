// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: virtual. can only be specified inside the class definition
// tidal-error-gcc: virtual. outside class declaration
// tidal-error-edg: invalid specifier outside a class declaration
struct S { virtual void f(); };
virtual void S::f() {}
