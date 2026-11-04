// tidal-rules: CTOR-17
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: backing array for .std::initializer_list. member
// tidal-note: Q-0084. The constructor uses the default member initializer, which sets an std::initializer_list member from a braced list. The backing array would end with the constructor. Clang rejects. GCC and EDG accept.
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
struct Sizes { std::initializer_list<int> list = {1, 2, 3}; Sizes() {} };
