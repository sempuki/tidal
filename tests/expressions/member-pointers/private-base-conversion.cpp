// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct B { int x; };
struct D : private B {};
int D::* pm = &B::x;
