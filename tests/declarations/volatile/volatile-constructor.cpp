// tidal-rules: VOL-7
// tidal-expect: reject
// tidal-error-clang: volatile. qualifier is not allowed on a constructor
// tidal-error-gcc: constructors may not be cv-qualified
// tidal-error-edg: type qualifier is not allowed on a constructor
struct S { S() volatile; };
