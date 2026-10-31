// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: typedef member .T. cannot be a bit-field
// tidal-error-gcc: cannot declare .T. to be a bit-field type
struct S { typedef int T : 3; };
