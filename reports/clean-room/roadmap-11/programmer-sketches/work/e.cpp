// A doubly linked List<T> with a sentinel node, an iterator template for
// mutable and const access (a converting constructor from the mutable one),
// the iterator operators, a range for, insert and erase that return
// iterators, splice, reverse, a merge sort that relinks nodes, a variadic
// constructor with a fold, and generic algorithms over any iterator pair.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
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
    Iter() = default;
    explicit Iter(Base* n) : n_(n) {}
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : n_(o.n_) {}
    Ref operator*() const { return static_cast<typename cond<Const, const Node*, Node*>::type>(n_)->value; }
    Ptr operator->() const { return &**this; }
    Iter& operator++() { n_ = n_->next; return *this; }
    Iter operator++(int) { Iter t = *this; ++*this; return t; }
    Iter& operator--() { n_ = n_->prev; return *this; }
    friend bool operator==(const Iter& a, const Iter& b) { return a.n_ == b.n_; }
    template <bool> friend class Iter;
  };
  using iterator = Iter<false>;
  using const_iterator = Iter<true>;

  List() = default;
  template <class... U> explicit List(U&&... us) { (push_back(forward<U>(us)), ...); }
  List(const List& o) : List() { for (const T& v : o) push_back(v); }
  List& operator=(const List&) = delete;
  ~List() { clear(); }

  iterator begin() { return iterator(head_.next); }
  iterator end() { return iterator(&head_); }
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
    unlink(n);
    delete static_cast<Node*>(n);
    --size_;
    return iterator(next);
  }
  void clear() { while (size_) erase(begin()); }
  void splice(const_iterator pos, List& other) {
    while (other.size_) {
      NodeBase* n = other.head_.next;
      unlink(n); --other.size_;
      link_before(const_cast<NodeBase*>(pos.n_), n); ++size_;
    }
  }
  void reverse() {
    NodeBase* n = &head_;
    do { NodeBase* t = n->next; n->next = n->prev; n->prev = t; n = t; } while (n != &head_);
  }
  template <class Less> void sort(Less less) {
    if (size_ < 2) return;
    List right;
    std::size_t half = size_ / 2;
    while (size_ > half) {
      NodeBase* n = head_.prev;
      unlink(n); --size_;
      link_before(right.head_.next, n); ++right.size_;
    }
    sort(less);
    right.sort(less);
    iterator a = begin();
    while (right.size_) {
      NodeBase* n = right.head_.next;
      while (a != end() && !less(static_cast<Node*>(n)->value, *a)) ++a;
      unlink(n); --right.size_;
      link_before(a.n_, n); ++size_;
    }
  }
};

template <class It, class P> It find_if(It first, It last, P p) {
  for (; first != last; ++first) if (p(*first)) return first;
  return last;
}
template <class It, class V, class Op> V accumulate(It first, It last, V init, Op op) {
  for (; first != last; ++first) init = op(init, *first);
  return init;
}
template <class It> std::size_t distance(It first, It last) {
  std::size_t n = 0;
  for (; first != last; ++first) ++n;
  return n;
}

struct Item {
  int key; int order;
  Item(int k, int o) : key(k), order(o) { ++live; }
  Item(const Item& o) : key(o.key), order(o.order) { ++live; }
  ~Item() { --live; }
};

int main() { List<int> a(5, 3); const List<int>& ca = a; return ca.begin() == ca.end() ? 1 : 0; }
