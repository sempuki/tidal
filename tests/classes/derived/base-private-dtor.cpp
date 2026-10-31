// tidal-rules: DER-21
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor
// tidal-error-gcc: use of deleted function .*D::D\(\)
// tidal-error-edg: default constructor of .D. cannot be referenced
class B { ~B() {} };
struct D : B {};
void f() { D d; }
