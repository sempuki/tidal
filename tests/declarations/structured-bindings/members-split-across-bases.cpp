// tidal-rules: SB-3
// tidal-expect: reject
struct B { int a; };
struct D : B { int b; };
D d;
auto [x, y] = d;
