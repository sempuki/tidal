struct Base { int v; };
struct Derived : Base {};
template <class Tag> struct Iter {
  Base* p;
  explicit Iter(Base* q) : p(q) {}
  template <class U> Iter(const Iter<U>& o) : p(o.p) {}
};
int take(Iter<int> it) { return it.p->v; }
int main() {
  Derived d{{0}};
  Iter<char> m(&d);
  return take(m);
}
