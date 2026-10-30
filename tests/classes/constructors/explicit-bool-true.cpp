// tidal-rules: CTOR-9
// tidal-expect: reject
struct S { explicit(true) S(int) {} };
S s = 1;
