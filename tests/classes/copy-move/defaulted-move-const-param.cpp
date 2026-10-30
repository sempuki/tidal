// tidal-rules: CPY-14
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-note: Q-0055. GCC 14 rejects a defaulted declaration whose parameter type differs from the implicit one. GCC 16, Clang, and EDG define it as deleted.
struct S { S() {} S(const S&&) = default; };
