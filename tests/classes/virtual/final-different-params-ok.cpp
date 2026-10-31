// tidal-rules: VIRT-3
// tidal-expect: accept
struct B { virtual void f() final; };
struct D : B { void f(int); };
