// tidal-rules: CTOR-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct B { B() {} B(int) {} };
struct D : B { using B::B; };
D d;
D e(1);
