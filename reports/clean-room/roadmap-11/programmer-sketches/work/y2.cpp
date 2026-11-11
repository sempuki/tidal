struct Node { int v; };
template <bool Const> struct Iter {
  int* p;
  explicit Iter(int* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
int take(Iter<true> it) { return *it.p; }
int main() {
  Node n{0};
  Iter<false> m(&n.v);
  return take(m);
}
