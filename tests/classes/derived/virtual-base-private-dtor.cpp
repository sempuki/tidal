// tidal-rules: DER-14
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor
// tidal-error-gcc: use of deleted function .*D::D\(\)
// tidal-error-edg: default constructor of .D. cannot be referenced
class A { ~A() {} friend struct B; };
struct B : virtual A {};
struct D : B {};
void f() { D d; }
