// tidal-rules: CONV-15
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct D : B { int d; };
int D::* pm = &B::b;
