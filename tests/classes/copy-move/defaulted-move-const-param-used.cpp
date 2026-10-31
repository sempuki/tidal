// tidal-rules: CPY-26
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted
// tidal-error-gcc: does not match the expected signature|use of deleted function
// tidal-error-edg: deleted function
struct S { S() {} S(const S&&) = default; };
const S a;
S b(static_cast<const S&&>(a));
