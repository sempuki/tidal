// tidal-rules: NAM-10
// tidal-expect: reject
// tidal-error-clang: unknown type name .T
// tidal-error-gcc: T. does not name a type
// tidal-error-edg: identifier .T. is undefined
struct S { T t; using T = int; };
