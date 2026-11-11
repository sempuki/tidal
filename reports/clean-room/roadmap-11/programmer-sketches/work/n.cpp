template <class T> struct remove_ref { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <bool B, class T, class F> struct cond { using type = T; };
template <class T, class F> struct cond<false, T, F> { using type = F; };
template <class T> class List {
  struct NodeBase { NodeBase* next; };
  struct Node : NodeBase {
    T value;
    Node(T v) : NodeBase{nullptr}, value(v) {}
  };
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
  template <class... A> iterator emplace(const_iterator pos, A&&... a) {
    Node* n = new Node(forward<A>(a)...);

    return iterator(n);
  }
  template <class U> void push_back(U&& u) { emplace(end(), forward<U>(u)); }
};
int main() {
  List<int> a;
  a.push_back(1);
}
