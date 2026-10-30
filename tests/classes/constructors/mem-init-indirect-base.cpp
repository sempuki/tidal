// tidal-rules: CTOR-15
// tidal-expect: reject
struct A { A(int) {} };
struct B : A { B() : A(1) {} };
struct C : B { C() : A(2) {} };
