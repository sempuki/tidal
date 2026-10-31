// tidal-rules: SB-8
// tidal-expect: reject
// tidal-error-clang: cannot appear in the initializer of its own
// tidal-error-gcc: use of .x. before deduction of .auto
// tidal-error-edg: cannot appear in its own initializer
int a[2];
auto [x, y] = (x, a);
