// tidal-rules: VIRT-8
// tidal-expect: accept
struct A { virtual void f() = 0; };
A make();
void take(A);
