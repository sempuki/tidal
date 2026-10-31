// tidal-rules: TYN-10
// tidal-expect: reject
// tidal-error-clang: tag type that does not match previous declaration
// tidal-error-gcc: .union. tag used in naming .struct S
// tidal-error-edg: tag kind of union is incompatible
struct S;
union S { int i; };
