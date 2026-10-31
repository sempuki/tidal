// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: overrides a .final. function
// tidal-error-gcc: overriding final function
// tidal-error-edg: cannot override .final. function
struct B { virtual void f() final; };
struct D : B { void f(); };
