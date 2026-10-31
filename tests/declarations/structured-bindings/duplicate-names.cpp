// tidal-rules: SB-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: redefinition of .auto x
// tidal-error-edg: incompatible with structured binding .x
int a[2];
auto [x, x] = a;
