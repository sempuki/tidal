// tidal-rules: DER-20
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor
// tidal-error-gcc: use of deleted function .*D::D\(\)
// tidal-error-edg: default constructor of .D. cannot be referenced
struct B { B(int) {} };
struct D : B { using B::B; };
D d;
