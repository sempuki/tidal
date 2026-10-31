// tidal-rules: VIRT-19
// tidal-expect: accept
struct B { virtual void f() = delete; };
struct D : B { void f() override = delete; };
