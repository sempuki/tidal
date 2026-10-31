// tidal-rules: ACC-13
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class B { int x = 1; };
class D : public B {
public:
  using B::x;
};
