// tidal-rules: PRIM-20
// tidal-expect: reject
// tidal-error-clang: invalid use of .this. outside of a non-static member function
// tidal-error-gcc: invalid use of .this. at top level
// tidal-error-edg: may only be used inside a nonstatic member function
auto p = this;
