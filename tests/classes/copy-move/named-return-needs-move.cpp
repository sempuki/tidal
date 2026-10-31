// tidal-rules: CPY-11
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct S { S() {} S(const S&) = delete; S(S&&) = delete; };
S make() { S s; return s; }
