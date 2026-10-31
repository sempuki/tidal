// tidal-rules: ILC-3
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
int total = 0;
struct S { S(std::initializer_list<int> l) { for (int v : l) total += v; } };
int main() { S s = {1, 2, 3}; return total == 6 ? 0 : 1; }
