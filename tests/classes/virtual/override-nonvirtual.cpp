// tidal-rules: VIRT-3
// tidal-expect: reject
struct B { void f(); };
struct D : B { void f() override; };
