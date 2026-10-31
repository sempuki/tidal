// tidal-rules: UNI-3
// tidal-expect: reject
struct S { ~S() {} };
union U { S s; int i; U() : i(0) {} };
void f() { U u; }
