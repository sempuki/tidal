// tidal-rules: CTOR-5
// tidal-expect: reject
struct S { int& r; S() : r() {} };
