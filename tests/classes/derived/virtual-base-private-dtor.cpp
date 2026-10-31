// tidal-rules: DER-7
// tidal-expect: reject
class A { ~A() {} friend struct B; };
struct B : virtual A {};
struct D : B {};
void f() { D d; }
