// tidal-rules: MPTR-2
// tidal-expect: reject
struct B { int x : 3; };
auto pm = &B::x;
