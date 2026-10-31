// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: qualifier is not allowed on a destructor
// tidal-error-gcc: destructors may not be cv-qualified
// tidal-error-edg: type qualifier is not allowed on a constructor or destructor
struct S { ~S() const; };
