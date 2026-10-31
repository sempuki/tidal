// tidal-rules: UNI-3
// tidal-expect: accept
struct S { S() {} ~S() {} };
union U { int i = 0; S s; ~U() {} };
void f() { U u; }
