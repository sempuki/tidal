// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: static member .n. cannot be a bit-field
// tidal-error-gcc: static member .n. cannot be a bit-field
struct S { static int n : 3; };
