struct Base { int v; };
struct Derived : Base {};
template <bool Const> struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
int take(Iter<true> it) { return it.p->v; }
int main() {
  Base b{0};
  Iter<false> m(&b);
  return take(m);
}
