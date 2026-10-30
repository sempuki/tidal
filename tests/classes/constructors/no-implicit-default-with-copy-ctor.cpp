// tidal-rules: CTOR-2
// tidal-expect: reject
struct S { S(const S&); };
S s;
