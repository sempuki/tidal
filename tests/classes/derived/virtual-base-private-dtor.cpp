// tidal-rules: DER-14
// tidal-expect: reject
class A { ~A() {} friend struct B; };
struct B : virtual A {};
struct D : B {};
void f() { D d; }
