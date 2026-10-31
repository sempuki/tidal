// tidal-rules: CPY-23
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct M { M() {} M(const M&) = delete; };
struct S { M m; S() {} S(const S&) = default; };
S a;
S b(a);
