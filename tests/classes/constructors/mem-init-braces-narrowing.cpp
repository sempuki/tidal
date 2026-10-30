// tidal-rules: CTOR-5
// tidal-expect: reject
struct S { int a; S(double d) : a{d} {} };
