// tidal-rules: SB-3
// tidal-expect: reject
struct S { union { int a; }; int b; };
S s;
auto [x, y] = s;
