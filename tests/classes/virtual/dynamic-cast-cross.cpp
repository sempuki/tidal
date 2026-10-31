// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct B { virtual ~B() {} int b = 4; };
struct D : A, B {};
int main() { D d; A* a = &d; B* b = dynamic_cast<B*>(a); return b && b->b == 4 ? 0 : 1; }
