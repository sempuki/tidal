// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct B { virtual void f(); };
struct D : B {};
void drop(B* p) { delete p; }
