// tidal-rules: CONV-17
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct B { int x; };
struct D : virtual B {};
int D::* pm = &B::x;
