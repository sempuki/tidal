// tidal-rules: CPY-1
// tidal-expect: reject
struct S { S() {} S(S&, int = 0) {} };
const S a;
S b(a);
