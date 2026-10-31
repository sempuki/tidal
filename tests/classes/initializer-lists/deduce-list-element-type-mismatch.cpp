// tidal-rules: ILC-10
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .count
// tidal-error-gcc: no matching function for call to .count
// tidal-error-edg: no instance of function template .count
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
template <class T> int count(std::initializer_list<T> l) { return int(l.size()); }
int x = count({1, 2.0});
