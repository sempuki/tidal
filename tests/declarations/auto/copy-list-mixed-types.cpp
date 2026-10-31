// tidal-rules: AUTO-7
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types
// tidal-error-gcc: unable to deduce .*initializer_list<auto>
// tidal-error-edg: cannot deduce .auto. type
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
auto a = {1, 2.0};
