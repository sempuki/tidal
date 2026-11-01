// tidal-rules: DED-16
// tidal-expect: accept
// tidal-note: Each element deduces T on its own, also through a const reference to the list type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
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
template <class T> Tag<T> list(std::initializer_list<T>);
template <class T> Tag<T> list_ref(const std::initializer_list<T>&);
static_assert(same<decltype(list({1, 2, 3})), Tag<int>>);
static_assert(same<decltype(list({'a'})), Tag<char>>);
static_assert(same<decltype(list_ref({1.0, 2.0})), Tag<double>>);
