// tidal-rules: CTOR-15
// tidal-expect: reject
// tidal-error-clang: multiple initializations given
// tidal-error-gcc: multiple initializations given
// tidal-error-edg: has already been initialized
struct S { int a; S() : a(1), a(2) {} };
