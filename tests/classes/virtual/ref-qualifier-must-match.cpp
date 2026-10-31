// tidal-rules: VIRT-1
// tidal-expect: reject
struct B { virtual int f() &; };
struct D : B { int f() && override; };
