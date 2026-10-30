// tidal-rules: DTOR-8
// tidal-expect: reject
struct S { ~S() {} };
struct T { ~T() {} };
void f(S& s) { s.~T(); }
