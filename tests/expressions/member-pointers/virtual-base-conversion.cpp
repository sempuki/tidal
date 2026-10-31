// tidal-rules: MPTR-4
// tidal-expect: reject
struct B { int x; };
struct D : virtual B { };
int D::* pm = &B::x;
