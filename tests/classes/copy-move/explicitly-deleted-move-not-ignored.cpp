// tidal-rules: CPY-7
// tidal-expect: reject
struct S { S() {} S(const S&) {} S(S&&) = delete; };
S a;
S b(static_cast<S&&>(a));
