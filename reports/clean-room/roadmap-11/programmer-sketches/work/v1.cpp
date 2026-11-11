struct Base { int v; };
struct Derived : Base {};
template <bool Const> struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
Derived d{{7}};
Iter<false> make(Iter<true> pos) { return Iter<false>(static_cast<Base*>(&d)); }
int main() {
  Iter<false> e(&d);
  return make(e).p->v == 7 ? 0 : 1;
}
