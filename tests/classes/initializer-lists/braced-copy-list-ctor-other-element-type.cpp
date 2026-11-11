// tidal-rules: ILC-3
// tidal-expect: run
// tidal-note: I7-17. Bag has an initializer-list constructor for int and its implicit copy constructor. In Bag b{a}, the first phase of ILC-3 finds no conversion from Bag to int, so the second phase takes the copy constructor. The same holds for the copy-list form. GCC, Clang, and EDG accept.
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
struct Bag {
  int count = 0;
  Bag(std::initializer_list<int> l) : count(static_cast<int>(l.size())) {}
};
int main() {
  Bag a{1, 2, 3};
  Bag b{a};
  Bag c = {a};
  return b.count == 3 && c.count == 3 ? 0 : 1;
}
