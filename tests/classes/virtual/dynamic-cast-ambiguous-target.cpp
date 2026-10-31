// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct B : A {};
struct C : A {};
struct D : B, C {};
int main() { D d; B* b = &d; A* a = b; return dynamic_cast<D*>(a) == &d ? 0 : 1; }
