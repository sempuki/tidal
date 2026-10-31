// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: only virtual member functions can be marked .override
// tidal-error-gcc: marked .override., but does not override
// tidal-error-edg: declared with .override. does not override
struct B { void f(); };
struct D : B { void f() override; };
