// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: ref-qualifier .&. is not allowed on a destructor
// tidal-error-gcc: destructors may not be ref-qualified
// tidal-error-edg: ref-qualifier is not allowed here
struct S { ~S() &; };
