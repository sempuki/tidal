// tidal-rules: VIRT-4
// tidal-expect: reject
struct B { virtual int f(); };
struct D : B { long f(); };
