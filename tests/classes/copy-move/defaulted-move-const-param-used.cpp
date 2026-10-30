// tidal-rules: CPY-14
// tidal-expect: reject
struct S { S() {} S(const S&&) = default; };
const S a;
S b(static_cast<const S&&>(a));
