// tidal-rules: CTOR-5
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
struct S { int a; S(double d) : a{d} {} };
