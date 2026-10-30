// tidal-rules: DTOR-6
// tidal-expect: reject
struct S { ~S() {} };
struct T { ~T() {} };
void f(S& s) { s.~T(); }
