// tidal-rules: VIRT-11
// tidal-expect: accept
struct A { virtual void f(); };
struct B : A {};
struct C { int v; };
B* to_b(A* a) { return dynamic_cast<B*>(a); }
