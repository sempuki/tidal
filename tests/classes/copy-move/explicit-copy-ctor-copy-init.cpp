// tidal-rules: CPY-9
// tidal-expect: reject
struct S { S() {} explicit S(const S&) {} };
S a;
S b = a;
