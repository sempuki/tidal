// tidal-rules: VIRT-8
// tidal-expect: reject
struct A { virtual void f() = 0; };
struct S { A a; };
