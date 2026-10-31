// tidal-rules: DER-15
// tidal-expect: reject
struct A { A() {} A(int) {} };
struct B { B() {} B(int) {} };
struct D : A, B { using A::A; using B::B; };
D d(1);
