// tidal-rules: DTOR-1
// tidal-expect: reject
struct S { ~S(int); };
