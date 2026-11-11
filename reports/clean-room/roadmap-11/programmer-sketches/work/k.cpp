template <bool B, class T, class F> struct cond { using type = T; };
template <class T, class F> struct cond<false, T, F> { using type = F; };
template <class T> class List {
  struct NodeBase { NodeBase* next; };
  NodeBase head_{&head_};
public:
  template <bool Const> class Iter {
    using Base = typename cond<Const, const NodeBase, NodeBase>::type;
    Base* n_ = nullptr;
    friend class List;
    explicit Iter(Base* n) : n_(n) {}
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : n_(o.n_) {}
  };
  using iterator = Iter<false>;
  using const_iterator = Iter<true>;
  iterator end() { return iterator(&head_); }
  const_iterator begin() const { return const_iterator(head_.next); }
  template <class U> int emplace(const_iterator pos, U) { return pos.n_ == &head_ ? 0 : 1; }
  template <class U> int push_back(U u) { return emplace(end(), u); }
};
int main() {
  List<int> a;
  return a.push_back(1);
}
