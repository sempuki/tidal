// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: must be the only declaration in its group
// tidal-error-edg: cannot be combined with other declarators
int a[2];
auto [x, y] = a, z = 1;
