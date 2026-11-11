// CTAD-18, ILC-3: the initializer-list guide and the copy candidate.
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr decltype(sizeof 0) size() const { return n_; }
};
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Bag {
  int count;
  constexpr Bag(std::initializer_list<T> l) : count(static_cast<int>(l.size())) {}
};
int main() {
  Bag a{1, 2, 3};
  Bag b{a};
  Bag c{a, a};
  static_assert(same<decltype(a), Bag<int>> && same<decltype(b), Bag<int>> && same<decltype(c), Bag<Bag<int>>>);
  return a.count == 3 && b.count == 3 && c.count == 2 ? 0 : 1;
}
