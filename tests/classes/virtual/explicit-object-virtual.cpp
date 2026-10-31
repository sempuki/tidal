// tidal-rules: VIRT-2
// tidal-expect: reject
struct S { virtual void f(this S&); };
