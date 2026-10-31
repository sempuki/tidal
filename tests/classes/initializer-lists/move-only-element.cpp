// tidal-rules: ILC-9
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted copy constructor
// tidal-error-gcc: use of deleted function .*M::M\(const M&\)
// tidal-error-edg: deleted function
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
struct M { M() {} M(M&&) {} };
void take(std::initializer_list<M>) {}
void f() { M m; take({static_cast<M&&>(m)}); M n = *std::initializer_list<M>{M{}}.begin(); }
