// tidal-rules: AUTO-7
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
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
auto a = {1, 2};
auto b = {1};
static_assert(same<decltype(a), std::initializer_list<int>>);
static_assert(same<decltype(b), std::initializer_list<int>>);
