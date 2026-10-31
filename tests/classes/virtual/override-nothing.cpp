// tidal-rules: VIRT-3
// tidal-expect: reject
struct B { virtual void f(int); };
struct D : B { void f(long) override; };
