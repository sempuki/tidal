// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: virtual. can only appear on non-static member functions
// tidal-error-gcc: virtual. outside class declaration
// tidal-error-edg: invalid specifier outside a class declaration
virtual void f();
