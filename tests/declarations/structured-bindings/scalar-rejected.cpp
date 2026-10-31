// tidal-rules: SB-13
// tidal-expect: reject
// tidal-error-clang: non-class, non-array type .int
// tidal-error-gcc: non-array non-class type .int
// tidal-error-edg: int. has no components to bind to
int i;
auto [x] = i;
