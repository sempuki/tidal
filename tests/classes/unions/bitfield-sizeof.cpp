// tidal-rules: BF-4
// tidal-expect: reject
struct S { int x : 3; };
auto n = sizeof(S{}.x);
