// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: only variables can be initialized
// tidal-error-gcc: typedef .A. is initialized
typedef int A = 3;
