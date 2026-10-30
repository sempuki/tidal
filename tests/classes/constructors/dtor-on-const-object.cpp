// tidal-rules: DTOR-2
// tidal-expect: accept
struct S { int v = 0; ~S() {} };
const S s;
