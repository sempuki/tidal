// tidal-rules: CPY-11
// tidal-expect: reject
struct S { S() {} S(const S&) = delete; S(S&&) = delete; };
S make() { S s; return s; }
