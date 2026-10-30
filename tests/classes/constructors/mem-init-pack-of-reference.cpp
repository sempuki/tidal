// tidal-rules: CTOR-15
// tidal-expect: reject
struct S { int& r; S() : r() {} };
