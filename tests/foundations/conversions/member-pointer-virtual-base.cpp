// tidal-rules: CONV-17
// tidal-expect: reject
// tidal-error-clang: via virtual base
// tidal-error-gcc: via virtual base
// tidal-error-edg: base class is virtual
// tidal-note: Gap review 2026-10-31 (third).
struct B { int x; };
struct D : virtual B {};
int D::* pm = &B::x;
