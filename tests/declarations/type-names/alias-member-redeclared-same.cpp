// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .T
// tidal-error-gcc: redeclaration of .using S::T
// tidal-error-edg: member typedef may not be redeclared
struct S { using T = int; using T = int; };
