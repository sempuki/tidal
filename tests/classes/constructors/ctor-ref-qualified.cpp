// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: ref-qualifier .&. is not allowed on a constructor
// tidal-error-gcc: constructors may not be ref-qualified
// tidal-error-edg: ref-qualifier is not allowed here
struct S { S() &; };
