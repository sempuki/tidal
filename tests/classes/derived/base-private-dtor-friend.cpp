// tidal-rules: DER-21
// tidal-expect: accept
// tidal-note: The access rules decide DER-21. B is a friend of A, so B has access to the private destructor of its virtual base A, and the special members of B are not deleted.
class A { ~A() {} friend struct B; };
struct B : virtual A {};
void f() { B b; }
