// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error-clang: must be in a namespace or global scope
// tidal-error-gcc: must be a non-member function
// tidal-error-edg: literal operator cannot be a member of a class
struct S { int operator""_m(unsigned long long); };
