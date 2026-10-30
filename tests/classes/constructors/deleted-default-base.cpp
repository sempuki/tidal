// tidal-rules: CTOR-3
// tidal-expect: reject
struct B { B(int); };
struct S : B {};
S s;
