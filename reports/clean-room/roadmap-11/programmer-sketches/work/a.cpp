template <bool B, class T, class F> struct cond { using type = T; };
template <class T, class F> struct cond<false, T, F> { using type = F; };
template <class T> struct List {
  struct NodeBase { NodeBase* next; };
  struct Node : NodeBase { T value; };
  NodeBase head_{&head_};
  template <bool Const> struct Iter {
    using Base = typename cond<Const, const NodeBase, NodeBase>::type;
    Base* n_ = nullptr;
    Iter() = default;
    explicit Iter(Base* n) : n_(n) {}
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : n_(o.n_) {}
    const T& operator*() const { return static_cast<typename cond<Const, const Node*, Node*>::type>(n_)->value; }
  };
  Iter<true> begin() const { return Iter<true>(head_.next); }
};
int main() { List<int> l; return l.begin().n_ == &l.head_ ? 0 : 1; }
