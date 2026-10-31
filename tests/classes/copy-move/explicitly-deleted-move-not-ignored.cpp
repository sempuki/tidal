// tidal-rules: CPY-23
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct S { S() {} S(const S&) {} S(S&&) = delete; };
S a;
S b(static_cast<S&&>(a));
