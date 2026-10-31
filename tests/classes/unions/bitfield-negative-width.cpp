// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: has negative width
// tidal-error-gcc: negative width in bit-field
// tidal-error-edg: invalid size for bit field
struct S { int x : -1; };
