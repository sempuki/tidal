// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .static. declaration specifier
// tidal-error-gcc: typedef. specifier conflicts with .static
// tidal-error-edg: more than one storage class
static typedef int T;
