// tidal-rules: VIRT-2
// tidal-expect: reject
struct B { virtual int f() const { return 1; } };
struct D : B { consteval int f() const override { return 2; } };
