// tidal-rules: CTOR-19
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .S
// tidal-error-gcc: no matching function for call to .S::S\(\)
// tidal-error-edg: no default constructor exists for class .S
struct S { S(const S&); };
S s;
