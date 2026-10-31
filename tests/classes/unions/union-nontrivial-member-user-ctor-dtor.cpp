// tidal-rules: UNI-3
// tidal-expect: accept
struct S { S() {} ~S() {} };
union U { S s; int i; U() : i(0) {} ~U() {} };
U u;
