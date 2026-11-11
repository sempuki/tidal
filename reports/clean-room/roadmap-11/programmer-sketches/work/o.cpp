template <class T> struct List {
  struct NodeBase { NodeBase* next; };
  struct Node : NodeBase { T value; };
  struct Iter {
    NodeBase* n_;
    explicit Iter(NodeBase* n) : n_(n) {}
  };
  Iter make(Node* n) { return Iter(n); }
};
int main() {
  List<int> l;
  List<int>::Node node{{nullptr}, 4};
  return l.make(&node).n_ == &node ? 0 : 1;
}
