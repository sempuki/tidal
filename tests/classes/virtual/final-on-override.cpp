// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: overrides a .final. function
// tidal-error-gcc: overriding final function
// tidal-error-edg: cannot override .final. function
struct A { virtual void f(); };
struct B : A { void f() override final; };
struct C : B { void f(); };
