// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: is a constructor name
// tidal-error-gcc: taking address of constructor
// tidal-error-edg: constructor or destructor may not have its address taken
struct C { C(); };
auto p = &C::C;
