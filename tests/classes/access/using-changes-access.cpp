// tidal-rules: ACC-9
// tidal-expect: accept
class B { protected: int x = 1; };
class D : public B {
public:
  using B::x;
};
int f(D& d) { return d.x; }
