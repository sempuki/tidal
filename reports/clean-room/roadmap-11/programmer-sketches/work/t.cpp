template <bool B, class T, class F> struct cond { using type = T; };
template <class T, class F> struct cond<false, T, F> { using type = F; };
struct NodeBase { NodeBase* next; };
struct Node : NodeBase { int value; };
template <bool Const> struct Iter {
  using Base = typename cond<Const, const NodeBase, NodeBase>::type;
  Base* n_;
  explicit Iter(Base* n) : n_(n) {}
};
int main() {
  Node node{{nullptr}, 4};
  Iter<false> it(&node);
  return it.n_ == &node ? 0 : 1;
}
