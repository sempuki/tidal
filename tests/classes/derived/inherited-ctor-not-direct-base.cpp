// tidal-rules: DER-9
// tidal-expect: reject
// tidal-error-clang: is not a direct base of .D., cannot inherit constructors
// tidal-error-gcc: is not a direct base of
// tidal-error-edg: must be inherited from a direct base class
struct A { A(int) {} };
struct B : A { using A::A; };
struct D : B { using A::A; };
