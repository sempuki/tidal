// tidal-rules: ILC-5
// tidal-expect: run
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
int sum(std::initializer_list<int> l) { int s = 0; for (int v : l) s += v; return s; }
int main() { std::initializer_list<int> l = {4, 5, 6}; return sum(l) == 15 ? 0 : 1; }
