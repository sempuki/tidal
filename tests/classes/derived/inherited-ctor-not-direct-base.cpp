// tidal-rules: DER-9
// tidal-expect: reject
struct A { A(int) {} };
struct B : A { using A::A; };
struct D : B { using A::A; };
