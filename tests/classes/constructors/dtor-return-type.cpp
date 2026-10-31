// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: destructor cannot have a return type
// tidal-error-gcc: return type specification for destructor invalid
// tidal-error-edg: return type may not be specified on a destructor
struct S { int ~S(); };
