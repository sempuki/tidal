// tidal-rules: VIRT-8
// tidal-expect: reject
struct A { virtual void f() = 0; };
void take(A a) {}
