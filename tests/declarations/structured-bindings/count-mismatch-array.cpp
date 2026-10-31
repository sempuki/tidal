// tidal-rules: SB-2
// tidal-expect: reject
// tidal-error-clang: but only 2 names were provided
// tidal-error-gcc: only 2 names provided for structured binding
// tidal-error-edg: more elements than there are binding names
int a[3];
auto [x, y] = a;
