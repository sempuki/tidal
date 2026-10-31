// tidal-rules: UNI-3
// tidal-expect: reject
struct S { S() {} ~S() {} };
union U { S s; int i; };
U u;
