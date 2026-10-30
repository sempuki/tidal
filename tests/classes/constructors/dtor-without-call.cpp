// tidal-rules: DTOR-8
// tidal-expect: reject
struct S { ~S() {} };
void f(S& s) { s.~S; }
