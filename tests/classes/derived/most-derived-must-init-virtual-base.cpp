// tidal-rules: DER-13
// tidal-expect: reject
// tidal-error-clang: must explicitly initialize the base class .A
// tidal-error-gcc: no matching function for call to .A::A\(\)
// tidal-error-edg: no default constructor exists for class .A
struct A { A(int) {} };
struct B : virtual A { B() : A(1) {} };
struct D : B { D() {} };
