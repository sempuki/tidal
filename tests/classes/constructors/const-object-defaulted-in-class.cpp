// tidal-rules: CTOR-4
// tidal-expect: reject
struct S { S() = default; int v; };
const S s;
