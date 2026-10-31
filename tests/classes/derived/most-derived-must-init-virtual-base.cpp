// tidal-rules: DER-13
// tidal-expect: reject
struct A { A(int) {} };
struct B : virtual A { B() : A(1) {} };
struct D : B { D() {} };
