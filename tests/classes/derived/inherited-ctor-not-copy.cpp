// tidal-rules: DER-8
// tidal-expect: reject
struct B { B() {} B(const B&) {} };
struct D : B { using B::B; };
B b;
D d(b);
