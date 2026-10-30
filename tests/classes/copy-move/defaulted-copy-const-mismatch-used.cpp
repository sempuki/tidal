// tidal-rules: CPY-14
// tidal-expect: reject
struct M { M() {} M(M&) {} };
struct S { M m; S() {} S(const S&) = default; };
S a;
S b(a);
