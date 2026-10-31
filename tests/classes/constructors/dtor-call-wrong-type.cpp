// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: does not match the type .S. of the object being de
// tidal-error-gcc: the type being destroyed is .S., but the destructor refers to .T
// tidal-error-edg: invalid destructor name for type .S
struct S { ~S() {} };
struct T { ~T() {} };
void f(S& s) { s.~T(); }
