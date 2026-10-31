// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-error-clang: not a power of 2
// tidal-error-gcc: not a positive power of 2
// tidal-error-edg: invalid argument to attribute .alignas
alignas(-8) int x;
