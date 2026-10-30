// tidal-rules: DTOR-6
// tidal-expect: accept
struct S { int v = 0; ~S() {} };
const S s;
