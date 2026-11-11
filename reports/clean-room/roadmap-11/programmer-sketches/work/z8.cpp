struct Base { int v; };
struct Derived : Base {};
struct Const;
struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
};
struct CIter {
  Base* p;
  CIter(const Iter& o) : p(o.p) {}
};
int take(CIter it) { return it.p->v; }
int main() {
  Derived d{{0}};
  Iter m(&d);
  return take(m);
}
