// tidal-rules: VIRT-9
// tidal-expect: reject
struct S { friend void f() = 0; };
