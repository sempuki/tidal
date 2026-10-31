// tidal-rules: VIRT-19
// tidal-expect: reject
struct B { virtual void f(); };
struct D : B { void f() override = delete; };
