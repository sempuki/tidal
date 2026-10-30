// tidal-rules: DTOR-3
// tidal-expect: reject
struct S { ~S() = delete; };
void f() { S s; }
