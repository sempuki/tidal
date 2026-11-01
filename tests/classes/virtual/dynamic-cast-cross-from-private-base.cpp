// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct B { virtual ~B() {} };
struct D : private A, public B { A* hidden() { return this; } };
int main() { D d; return dynamic_cast<B*>(d.hidden()) == nullptr ? 0 : 1; }
