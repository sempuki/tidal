// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: cannot have a default argument
// tidal-error-gcc: cannot have default arguments
// tidal-error-edg: default argument is not allowed
struct S { S operator+(int = 1); };
