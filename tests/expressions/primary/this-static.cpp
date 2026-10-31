// tidal-rules: PRIM-20
// tidal-expect: reject
// tidal-error-clang: invalid use of .this. outside of a non-static member function
// tidal-error-gcc: unavailable for static member functions
// tidal-error-edg: may only be used inside a nonstatic member function
struct S { static S* f() { return this; } };
