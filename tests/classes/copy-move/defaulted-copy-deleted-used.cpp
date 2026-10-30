// tidal-rules: CPY-7
// tidal-expect: reject
struct M { M() {} M(const M&) = delete; };
struct S { M m; S() {} S(const S&) = default; };
S a;
S b(a);
