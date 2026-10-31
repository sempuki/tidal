// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: named bit-field .x. has zero width
// tidal-error-gcc: zero width for bit-field
// tidal-error-edg: zero-length bit field must be unnamed
struct S { int x : 0; };
