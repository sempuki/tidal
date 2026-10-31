// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: destructor cannot be declared .static
// tidal-error-gcc: destructor cannot be static member function
// tidal-error-edg: static. is not allowed
struct S { static ~S(); };
