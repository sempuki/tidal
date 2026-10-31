// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error-clang: literal operator must have C\+\+ linkage
// tidal-error-gcc: literal operator with C linkage
// tidal-error-edg: literal operator cannot have extern .C. name linkage
extern "C" int operator""_c(unsigned long long);
