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
    // Second version: Iter<!Const> in place of Iter<false>; the front end
    // finds no member n_ in Iter<false> named inside the template.
    template <bool C = Const> requires C
    Iter(const Iter<!Const>& o) : n_(o.n_) {}
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

int main() {
  int fails = 0;
  {
    List<int> a(5, 3, 8, 1);
    a.push_front(9);
    int sum = accumulate(a.begin(), a.end(), 0, [](int s, int v) { return s + v; });
    if (sum != 26 || a.size() != 5) ++fails;
    auto it = find_if(a.begin(), a.end(), [](int v) { return v > 6 && v < 9; });
    if (it == a.end() || *it != 8) ++fails;
    it = a.erase(it);
    if (*it != 1 || a.size() != 4) ++fails;
    a.reverse();
    int expect[] = {1, 3, 5, 9};
    int i = 0;
    for (int v : a) if (v != expect[i++]) ++fails;
    const List<int>& ca = a;
    List<int>::const_iterator c = a.begin();
    if (c != ca.begin() || distance(ca.begin(), ca.end()) != 4) ++fails;
  }
  {
    List<Item> items;
    items.emplace(items.end(), 2, 0);
    items.emplace(items.end(), 1, 1);
    items.emplace(items.end(), 2, 2);
    items.emplace(items.end(), 1, 3);
    List<Item> more;
    more.emplace(more.end(), 0, 4);
    items.splice(items.begin(), more);
    if (more.size() != 0 || items.size() != 5 || items.begin()->key != 0) ++fails;
    items.sort([](const Item& x, const Item& y) { return x.key < y.key; });
    int keys[] = {0, 1, 1, 2, 2}, orders[] = {4, 1, 3, 0, 2};
    int i = 0;
    for (const Item& it : items) { if (it.key != keys[i] || it.order != orders[i]) ++fails; ++i; }
    List<Item> copy = items;
    if (live != 10) ++fails;
  }
  if (live != 0) ++fails;
  return fails;
}
