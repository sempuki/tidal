struct Base { int v; };
struct Derived : Base {};
struct Ptr {
  Base* p;
  explicit Ptr(Base* q) : p(q) {}
};
struct CPtr {
  Base* p;
  template <class X = int> CPtr(const Ptr& o) : p(o.p) {}
};
int take(CPtr it) { return it.p->v; }
int main() {
  Derived d{{0}};
  Ptr m(&d);
  return take(m);
}
