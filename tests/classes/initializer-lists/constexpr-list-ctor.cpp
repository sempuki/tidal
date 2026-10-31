// tidal-rules: ILC-3
// tidal-expect: accept
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr decltype(sizeof 0) size() const { return n_; }
  constexpr const E* begin() const { return b_; }
  constexpr const E* end() const { return b_ + n_; }
};
}
struct Sum { int v = 0; constexpr Sum(std::initializer_list<int> l) { for (int x : l) v += x; } };
constexpr Sum s{1, 2, 3};
static_assert(s.v == 6);
