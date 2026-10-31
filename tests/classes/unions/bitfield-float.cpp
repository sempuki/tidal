// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: has non-integral type
// tidal-error-gcc: non-integral type
// tidal-error-edg: invalid type for a bit field
struct S { float f : 3; };
