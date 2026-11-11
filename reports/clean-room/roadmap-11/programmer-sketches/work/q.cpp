template <class T> struct List {
  struct NodeBase { NodeBase* next; };
  struct Node : NodeBase { T value; };
  template <bool Const> struct Iter {
    NodeBase* n_;
    explicit Iter(NodeBase* n) : n_(n) {}
    template <bool C = Const>
    Iter(const Iter<false>& o) : n_(o.n_) {}
  };
  Iter<false> make(Node* n) { return Iter<false>(n); }
};
int main() {
  List<int> l;
  List<int>::Node node{{nullptr}, 4};
  return l.make(&node).n_ == &node ? 0 : 1;
}
