// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: redefinition of .S
// tidal-error-gcc: redefinition of .struct S
// tidal-error-edg: has already been defined
struct S { int a; };
struct S { int a; };
