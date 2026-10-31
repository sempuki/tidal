// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: destructor cannot have any parameters|destructor cannot be variadic
// tidal-error-gcc: destructors may not have parameters
// tidal-error-edg: destructor may not have parameters
struct S { ~S(int); };
