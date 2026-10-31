// tidal-rules: VIRT-2
// tidal-expect: reject
struct B { virtual void f(); };
struct D : B { static void f(); };
