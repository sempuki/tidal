// tidal-rules: CPY-4
// tidal-expect: accept
struct S { S() {} S(S&&) {} S(const S&) = default; };
S a;
S b(a);
