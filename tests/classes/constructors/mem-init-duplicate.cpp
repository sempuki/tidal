// tidal-rules: CTOR-15
// tidal-expect: reject
struct S { int a; S() : a(1), a(2) {} };
