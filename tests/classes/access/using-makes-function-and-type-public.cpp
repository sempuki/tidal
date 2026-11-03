// tidal-rules: ACC-9
// tidal-expect: accept
// tidal-note: Using-declarations make a protected member function and a protected type alias public.
class B {
protected:
  int get() const { return 1; }
  using Kind = long;
};
class D : public B {
public:
  using B::get;
  using B::Kind;
};
D::Kind f(const D& d) { return d.get(); }
