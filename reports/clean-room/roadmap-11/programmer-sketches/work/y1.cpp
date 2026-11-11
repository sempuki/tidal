struct Node { int v; };
template <bool Const> struct Iter {
  Node* p;
  explicit Iter(Node* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
int take(Iter<true> it) { return it.p->v; }
int main() {
  Node n{0};
  Iter<false> m(&n);
  return take(m);
}
