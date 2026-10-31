// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct B { virtual void f(int); };
struct D : B { void f(const char*); };
