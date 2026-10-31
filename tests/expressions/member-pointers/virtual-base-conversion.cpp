// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: via virtual base
// tidal-error-gcc: via virtual base
// tidal-error-edg: base class is virtual
struct B { int x; };
struct D : virtual B { };
int D::* pm = &B::x;
