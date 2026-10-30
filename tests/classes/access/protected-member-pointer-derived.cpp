// tidal-rules: ACC-4
// tidal-expect: accept
class B { protected: int x = 1; };
class D : public B {
public:
  static int D::* ptr() { return &D::x; }
};
