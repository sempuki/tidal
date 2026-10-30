// tidal-rules: CPY-14
// tidal-expect: accept
struct S { S() {} S(S&) = default; };
S a;
S b(a);
