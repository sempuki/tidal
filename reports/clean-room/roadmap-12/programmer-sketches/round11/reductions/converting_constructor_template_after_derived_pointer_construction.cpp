// Iter<false> built from a Derived* through explicit Iter(Base*), then
// converted to Iter<true> through a constrained constructor template with a
// default template argument. From r10-list-first-try (TR-13 there).
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
  Derived d{{0}};
  Iter<false> m(&d);
  return take(m);
}
