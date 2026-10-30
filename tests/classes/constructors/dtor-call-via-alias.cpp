// tidal-rules: DTOR-6
// tidal-expect: accept
struct S { ~S() {} };
using A = S;
void f(S& s) { s.~A(); }
