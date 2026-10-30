// tidal-rules: CTOR-9
// tidal-expect: reject
struct S { S(int); };
explicit S::S(int) {}
