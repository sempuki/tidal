struct NodeBase { NodeBase* next; };
struct Node : NodeBase { int value; };
template <bool Const> struct Iter {
  NodeBase* n_;
  explicit Iter(NodeBase* n) : n_(n) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : n_(o.n_) {}
};
int main() {
  Node node{{nullptr}, 4};
  Iter<false> it(&node);
  return it.n_ == &node ? 0 : 1;
}
