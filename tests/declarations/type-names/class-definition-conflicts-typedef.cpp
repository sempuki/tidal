// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: conflicts with typedef of the same name
// tidal-error-gcc: using typedef-name .T. after .struct
// tidal-error-edg: invalid redeclaration of type name
typedef int T;
struct T { int v; };
