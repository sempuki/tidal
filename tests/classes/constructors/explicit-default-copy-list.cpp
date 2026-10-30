// tidal-rules: CTOR-9
// tidal-expect: reject
struct S { explicit S() {} };
S make() { return {}; }
