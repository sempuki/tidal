// tidal-rules: VIRT-8
// tidal-expect: accept
struct A { virtual void f() = 0; int v; };
static_assert(sizeof(A) >= sizeof(int));
