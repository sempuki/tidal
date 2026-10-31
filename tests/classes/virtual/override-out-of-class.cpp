// tidal-rules: VIRT-3
// tidal-expect: reject
struct B { virtual void f(); };
struct D : B { void f() override; };
void D::f() override {}
