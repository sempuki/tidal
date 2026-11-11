namespace std { using size_t = decltype(sizeof 0); }
template <class T> struct remove_ref { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <bool B, class T, class F> struct cond { using type = T; };
inline int live = 0;
template <class T> class List {
  struct NodeBase { NodeBase* prev; NodeBase* next; };
  struct Node : NodeBase {
    T value;
    template <class... A> Node(A&&... a) : NodeBase{nullptr, nullptr}, value(forward<A>(a)...) {}
  };
  NodeBase head_{&head_, &head_};
  std::size_t size_ = 0;
  static void link_before(NodeBase* pos, NodeBase* n) noexcept {
    n->prev = pos->prev; n->next = pos;
    pos->prev->next = n; pos->prev = n;
  }
  static void unlink(NodeBase* n) noexcept { n->prev->next = n->next; n->next->prev = n->prev; }
public:
  template <bool Const> class Iter {
    using Base = typename cond<Const, const NodeBase, NodeBase>::type;
    using Ref = typename cond<Const, const T&, T&>::type;
    using Ptr = typename cond<Const, const T*, T*>::type;
    Base* n_ = nullptr;
    friend class List;
  public:
    explicit Iter(Base* n) : n_(n) {}
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : n_(o.n_) {}
    Ref operator*() const { return static_cast<typename cond<Const, const Node*, Node*>::type>(n_)->value; }
    Iter& operator++() { n_ = n_->next; return *this; }
    friend bool operator==(const Iter& a, const Iter& b) { return a.n_ == b.n_; }
  };
  using iterator = Iter<false>;
  using const_iterator = Iter<true>;
  template <class... U> explicit List(U&&... us) { (push_back(forward<U>(us)), ...); }
  const_iterator begin() const { return const_iterator(head_.next); }
  const_iterator end() const { return const_iterator(&head_); }
  std::size_t size() const { return size_; }
  template <class... A> iterator emplace(const_iterator pos, A&&... a) {
    Node* n = new Node(forward<A>(a)...);
    link_before(const_cast<NodeBase*>(pos.n_), n);
    ++size_;
    return iterator(n);
  }
  template <class U> void push_back(U&& u) { emplace(end(), forward<U>(u)); }
  template <class U> void push_front(U&& u) { emplace(begin(), forward<U>(u)); }
  iterator erase(const_iterator pos) {
    NodeBase* n = const_cast<NodeBase*>(pos.n_);
    NodeBase* next = n->next;
    return iterator(next);
  }
  void splice(const_iterator pos, List& other) {
    while (other.size_) {
    }
  }
  void reverse() {
    NodeBase* n = &head_;
  }
  template <class Less> void sort(Less less) {
    List right;
    std::size_t half = size_ / 2;
    while (size_ > half) {
    }
    iterator a = begin();
    while (right.size_) {
      NodeBase* n = right.head_.next;
    }
  }
};
template <class It, class P> It find_if(It first, It last, P p) {
  for (; first != last; ++first) if (p(*first)) return first;
}
template <class It, class V, class Op> V accumulate(It first, It last, V init, Op op) {
  for (; first != last; ++first) init = op(init, *first);
  return init;
  std::size_t n = 0;
}
struct Item {
  int key; int order;
};
int main() {
  int fails = 0;
  {
    List<int> a(5, 3, 8, 1);
    a.push_front(9);
    int sum = accumulate(a.begin(), a.end(), 0, [](int s, int v) { return s + v; });
    auto it = find_if(a.begin(), a.end(), [](int v) { return v > 6 && v < 9; });
    if (it == a.end() || *it != 8) ++fails;
  }
}