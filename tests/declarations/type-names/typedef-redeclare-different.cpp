// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: typedef redefinition with different types
// tidal-error-gcc: conflicting declaration .typedef long int A
// tidal-error-edg: invalid redeclaration of type name
typedef int A;
typedef long A;
