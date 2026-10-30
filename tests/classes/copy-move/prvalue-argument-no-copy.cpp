// tidal-rules: CPY-10
// tidal-expect: accept
struct S { S() {} S(S&&) = delete; };
void take(S) {}
void f() { take(S()); }
