struct Base { int v; };
struct Derived : Base {};
template <bool Const> struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
  template <class X> Iter(const Iter<false>& o, X) : p(o.p) {}
  Iter(const Iter<!Const>& o, int = 0) : p(o.p) {}
};
int take(Iter<true> it) { return it.p->v; }
int main() {
  Derived d{{0}};
  Iter<false> m(&d);
  return take(m);
}
