// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: explicit. can only be specified inside the class definition
// tidal-error-gcc: explicit. outside class declaration
// tidal-error-edg: explicit. is not allowed
struct S { S(int); };
explicit S::S(int) {}
