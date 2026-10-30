// tidal-rules: DTOR-1
// tidal-expect: reject
struct S { int ~S(); };
