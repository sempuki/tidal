// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} int a; };
struct B { virtual ~B() {} int b; };
struct D : A, B {};
int main() { D d; B* b = &d; return dynamic_cast<void*>(b) == static_cast<void*>(&d) ? 0 : 1; }
