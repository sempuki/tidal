// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: typedef redefinition with different types
// tidal-error-gcc: redeclaration of .typedef long int S::T
// tidal-error-edg: invalid redeclaration of type name
struct S { typedef int T; typedef long T; };
