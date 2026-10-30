// tidal-rules: CPY-10
// tidal-expect: accept
struct S { S() {} S(const S&) = delete; S(S&&) = delete; };
S make() { return S(); }
S a = make();
S b = S(S(S()));
