// tidal-rules: SB-15
// tidal-expect: reject
// tidal-error-clang: with type .auto \*.; declared type must be .auto
// tidal-error-gcc: expected unqualified-id before .\[. token
// tidal-error-edg: invalid structured binding syntax
int a[2];
auto* [x, y] = a;
