// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: constructor cannot be declared .static
// tidal-error-gcc: constructor cannot be static member function
// tidal-error-edg: static. is not allowed
struct S { static S(); };
