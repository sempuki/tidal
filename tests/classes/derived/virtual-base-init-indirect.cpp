// tidal-rules: DER-7
// tidal-expect: accept
struct A { A(int) {} };
struct B : virtual A { B() : A(1) {} };
struct D : B { D() : A(2) {} };
