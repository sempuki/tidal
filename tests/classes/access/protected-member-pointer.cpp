// tidal-rules: ACC-4
// tidal-expect: reject
class B { protected: int x = 1; };
class D : public B {
public:
  static int B::* ptr() { return &B::x; }
};
