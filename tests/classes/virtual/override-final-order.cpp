// tidal-rules: VIRT-3
// tidal-expect: accept
struct A { virtual void f(); };
struct B : A { void f() final override; };
