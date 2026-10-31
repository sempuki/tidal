// tidal-rules: CTOR-2
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .S
// tidal-error-gcc: no matching function for call to .S::S\(\)
// tidal-error-edg: no default constructor exists for class .S
struct S { S(int); };
S s;
