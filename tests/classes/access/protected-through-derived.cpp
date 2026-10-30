// tidal-rules: ACC-4
// tidal-expect: accept
class B { protected: int x = 1; };
class D : public B {
public:
  int own() const { return x; }
  int other(const D& d) const { return d.x; }
};
