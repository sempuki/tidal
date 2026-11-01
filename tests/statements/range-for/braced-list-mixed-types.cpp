// tidal-rules: AUTO-21 RFOR-1
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types
// tidal-error-gcc: unable to deduce
// tidal-error-edg: deduce|initializer_list|conflicting
// tidal-note: The braced list deduces std::initializer_list<T> as auto does, and the elements give int and double.
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
int sum() {
  int total = 0;
  for (auto v : {1, 2.0}) total += int(v);
  return total;
}
