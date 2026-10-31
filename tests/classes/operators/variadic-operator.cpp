// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: cannot be variadic
// tidal-error-gcc: must not have variable number of arguments
// tidal-error-edg: ellipsis is not allowed in operator function parameter list
struct S { S operator+(...); };
