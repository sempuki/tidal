// tidal-rules: CTOR-15
// tidal-expect: reject
// tidal-error-clang: reference to type .int. requires an initializer
// tidal-error-gcc: value-initialization of reference type
// tidal-error-edg: default-initialization of reference is not allowed
struct S { int& r; S() : r() {} };
