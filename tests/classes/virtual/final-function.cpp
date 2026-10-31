// tidal-rules: VIRT-3
// tidal-expect: reject
struct B { virtual void f() final; };
struct D : B { void f(); };
