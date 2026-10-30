// tidal-rules: CPY-1
// tidal-expect: reject
struct S { S() {} S(S&) {} };
const S a;
S b(a);
