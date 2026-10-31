// tidal-rules: CPY-17
// tidal-expect: reject
// tidal-error-clang: would delete it after its first declaration
// tidal-error-gcc: use of deleted function .*M::M
// tidal-error-edg: deleted function
struct M { M() {} M(const M&) = delete; };
struct S { M m; S() {} S(const S&); };
S::S(const S&) = default;
