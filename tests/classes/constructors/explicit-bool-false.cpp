// tidal-rules: CTOR-9
// tidal-expect: accept
struct S { explicit(false) S(int) {} };
S s = 1;
