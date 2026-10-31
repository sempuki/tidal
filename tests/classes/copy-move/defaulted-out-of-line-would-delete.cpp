// tidal-rules: CPY-17
// tidal-expect: reject
struct M { M() {} M(const M&) = delete; };
struct S { M m; S() {} S(const S&); };
S::S(const S&) = default;
