// tidal-rules: DER-8
// tidal-expect: reject
struct B { B(int) {} };
struct D : B { using B::B; };
D d;
