// tidal-rules: CPY-14
// tidal-expect: reject
struct S { S() {} S(const volatile S&) = default; };
S a;
S b(a);
