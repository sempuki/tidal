// tidal-rules: ILC-8
// tidal-expect: reject
// tidal-error-clang: constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: cannot use a constructor marked .explicit
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
struct S { explicit S(std::initializer_list<int>) {} };
S s = {1, 2};
