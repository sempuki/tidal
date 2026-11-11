namespace std { using size_t = decltype(sizeof 0); }
template <class T> struct remove_ref { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <bool B, class T, class F> struct cond { using type = T; };
template <class T, class F> struct cond<false, T, F> { using type = F; };
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
  }
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
  template <class... A> iterator emplace(const_iterator pos, A&&... a) {
    Node* n = new Node(forward<A>(a)...);
    return iterator(n);
  }
  template <class U> void push_back(U&& u) { emplace(end(), forward<U>(u)); }
  iterator erase(const_iterator pos) {
  }
  void splice(const_iterator pos, List& other) {
    while (other.size_) {
      NodeBase* n = other.head_.next;
    }
    List right;
    iterator a = begin();
    while (right.size_) {
      NodeBase* n = right.head_.next;
    }
  }
};
template <class It, class P> It find_if(It first, It last, P p) {
}
template <class It, class V, class Op> V accumulate(It first, It last, V init, Op op) {
}
template <class It> std::size_t distance(It first, It last) {
}
struct Item {
};
int main() { List<int> a; a.push_back(1); return 0; }