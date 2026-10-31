// tidal-rules: DER-8
// tidal-expect: reject
struct A { A(int) {} };
struct B : A { using A::A; };
struct D : B { using A::A; };
