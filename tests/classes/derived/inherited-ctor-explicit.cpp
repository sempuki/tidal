// tidal-rules: DER-8
// tidal-expect: reject
struct B { explicit B(int) {} };
struct D : B { using B::B; };
D d = 1;
