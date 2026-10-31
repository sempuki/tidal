// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: marked .override. hides virtual member function
// tidal-error-gcc: marked .override., but does not override
// tidal-error-edg: declared with .override. does not override
struct B { virtual void f(int); };
struct D : B { void f(long) override; };
