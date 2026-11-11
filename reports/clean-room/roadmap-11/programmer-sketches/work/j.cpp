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
  Iter<false> end() { return Iter<false>(&head_); }
  Iter<true> begin() const { return Iter<true>(head_.next); }
  template <class U> int emplace(Iter<true> pos, U) { return pos.n_ == &head_ ? 0 : 1; }
  template <class U> int push_back(U u) { return emplace(end(), u); }
  void splice() {
    Iter<false> a = begin();
  }
};
int main() {
  List<int> a;
  return a.push_back(1);
}
