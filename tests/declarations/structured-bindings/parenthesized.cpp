// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: cannot be declared with parentheses
// tidal-error-gcc: expected unqualified-id before .\[. token
int a[2];
auto ([x, y]) = a;
