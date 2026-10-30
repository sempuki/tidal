// tidal-rules: RFOR-1
// tidal-expect: run
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr const E* begin() const { return b_; }
  constexpr const E* end() const { return b_ + n_; }
};
}
int main() {
  int sum = 0;
  for (int v : {1, 2, 3}) sum += v;
  return sum == 6 ? 0 : 1;
}
