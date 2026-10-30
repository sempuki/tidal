// tidal-rules: LIST-5
// tidal-expect: reject
struct P { int x; };
struct S { P p; };
S s{.p.x = 1};
