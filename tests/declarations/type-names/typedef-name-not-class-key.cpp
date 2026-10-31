// tidal-rules: TYN-4
// tidal-expect: reject
// tidal-error-clang: cannot be referenced with (a|the .)struct. specifier|cannot be referenced with a struct specifier
// tidal-error-gcc: using typedef-name .T. after .struct
// tidal-error-edg: may not be used in an elaborated type specifier
struct S {};
typedef S T;
struct T* p;
