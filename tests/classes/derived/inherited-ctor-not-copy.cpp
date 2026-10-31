// tidal-rules: DER-20
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .D
// tidal-error-gcc: no matching function for call to .D::D\(B&\)
// tidal-error-edg: no instance of constructor .D::D
struct B { B() {} B(const B&) {} };
struct D : B { using B::B; };
B b;
D d(b);
