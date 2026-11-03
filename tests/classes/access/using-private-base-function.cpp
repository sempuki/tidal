// tidal-rules: ACC-13
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: The using-declaration names a private member function of the base.
class B {
  void f() {}
};
class D : public B {
public:
  using B::f;
};
