struct Base { int v; };
struct Derived : Base {};
template <bool Const> struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
Derived d{{7}};
Iter<false> make(Iter<true> pos) { return Iter<false>(&d); }
int main() {
  Iter<false> e(&d); Iter<true> t = e;
  return make(t).p->v == 7 ? 0 : 1;
}
