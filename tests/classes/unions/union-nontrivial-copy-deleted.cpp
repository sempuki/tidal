// tidal-rules: UNI-3
// tidal-expect: reject
struct S { S() {} S(const S&) {} };
union U { S s; int i; U() : i(0) {} };
U a;
U b(a);
