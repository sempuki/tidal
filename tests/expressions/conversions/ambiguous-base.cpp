// tidal-rules: CONV-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct L : B {};
struct R : B {};
struct D : L, R {};
D d;
B* p = &d;
