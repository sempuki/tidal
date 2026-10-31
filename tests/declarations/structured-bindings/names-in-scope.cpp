// tidal-rules: SB-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .x. as different kind of symbol
// tidal-error-gcc: redefinition of .int x
// tidal-error-edg: incompatible with structured binding .x
int a[2];
auto [x, y] = a;
int x;
