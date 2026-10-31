// tidal-rules: VIRT-8
// tidal-expect: accept
struct A { virtual void f() = 0; };
A* p = nullptr;
void use(A& a) { a.f(); }
