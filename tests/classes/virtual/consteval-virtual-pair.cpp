// tidal-rules: VIRT-14
// tidal-expect: accept
struct B { consteval virtual int f() const { return 1; } };
struct D : B { consteval int f() const override { return 2; } };
