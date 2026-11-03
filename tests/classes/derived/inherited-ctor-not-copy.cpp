// tidal-rules: DER-20 DER-8
// tidal-expect: reject
// tidal-note: D inherits constructors, so D is not an aggregate, and D(b) has no parenthesized aggregate initialization to fall back on.
// tidal-error-clang: no matching constructor for initialization of .D
// tidal-error-gcc: no matching function for call to .D::D\(B&\)
// tidal-error-edg: no instance of constructor .D::D
struct B { B() {} B(const B&) {} };
struct D : B { using B::B; };
B b;
D d(b);
