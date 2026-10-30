// tidal-rules: CONV-15
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct D : B { int d; };
int B::* pm = &D::d;
