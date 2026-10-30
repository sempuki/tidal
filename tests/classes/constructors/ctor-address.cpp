// tidal-rules: CTOR-1
// tidal-expect: reject
struct S { S(); };
auto p = &S::S;
