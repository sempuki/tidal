// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: static. is not permitted on a declaration of a type
// tidal-error-gcc: storage class can only be specified for objects and functions
// tidal-error-edg: storage class may not be specified here
static struct S { int v; };
