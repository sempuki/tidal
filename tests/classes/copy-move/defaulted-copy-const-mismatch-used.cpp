// tidal-rules: CPY-14
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct M { M() {} M(M&) {} };
struct S { M m; S() {} S(const S&) = default; };
S a;
S b(a);
