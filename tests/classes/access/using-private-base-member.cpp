// tidal-rules: ACC-13
// tidal-expect: reject
class B { int x = 1; };
class D : public B {
public:
  using B::x;
};
