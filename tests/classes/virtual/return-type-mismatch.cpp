// tidal-rules: VIRT-15
// tidal-expect: reject
struct B { virtual int f(); };
struct D : B { long f(); };
