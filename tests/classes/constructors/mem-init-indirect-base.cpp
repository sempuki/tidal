// tidal-rules: CTOR-15
// tidal-expect: reject
// tidal-error-clang: is not a direct or virtual base
// tidal-error-gcc: is not a direct base
// tidal-error-edg: indirect nonvirtual base class is not allowed
struct A { A(int) {} };
struct B : A { B() : A(1) {} };
struct C : B { C() : A(2) {} };
