// tidal-rules: MPTR-4
// tidal-expect: reject
struct B { int x; };
struct D : B { int y; };
int B::* pm = &D::y;
