// tidal-rules: CPY-14
// tidal-expect: accept
struct M { M() {} M(M&) {} };
struct S { M m; S() {} S(const S&) = default; };
