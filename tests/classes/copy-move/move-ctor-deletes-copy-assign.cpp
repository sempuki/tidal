// tidal-rules: CPY-4
// tidal-expect: reject
struct S { S() {} S(S&&) {} };
void f(S& a, S& b) { a = b; }
