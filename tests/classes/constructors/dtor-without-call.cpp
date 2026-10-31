// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: reference to destructor must be called
// tidal-error-gcc: invalid use of non-static member function .S::~S
// tidal-error-edg: pointer to a bound function may only be used to call
struct S { ~S() {} };
void f(S& s) { s.~S; }
