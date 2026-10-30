// tidal-rules: DTOR-3
// tidal-expect: reject
class S { ~S() {} };
void f() { S s; }
