// tidal-rules: VIRT-3
// tidal-expect: reject
struct B { virtual int f() &; };
struct D : B { int f() && override; };
