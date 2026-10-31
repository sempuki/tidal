// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: declaration not permitted in this context
// tidal-error-gcc: expected unqualified-id before .\[. token
int a[2];
struct S { auto [x, y] = a; };
