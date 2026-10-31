// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: only virtual member functions can be marked .final
// tidal-error-gcc: marked .final., but is not virtual
// tidal-error-edg: nonvirtual function cannot be declared with .final
struct S { void f() final; };
