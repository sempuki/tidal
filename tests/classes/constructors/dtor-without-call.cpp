// tidal-rules: DTOR-6
// tidal-expect: reject
struct S { ~S() {} };
void f(S& s) { s.~S; }
