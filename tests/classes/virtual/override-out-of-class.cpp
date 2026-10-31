// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: override. specifier is not allowed outside a class definition
// tidal-error-gcc: virt-specifiers in .f. not allowed outside a class definition
struct B { virtual void f(); };
struct D : B { void f() override; };
void D::f() override {}
