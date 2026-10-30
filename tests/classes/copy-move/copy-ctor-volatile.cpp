// tidal-rules: CPY-1
// tidal-expect: accept
struct S { S() {} S(const volatile S&) {} };
volatile S a;
S b(a);
