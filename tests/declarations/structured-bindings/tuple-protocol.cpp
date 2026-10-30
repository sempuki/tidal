// tidal-rules: SB-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct Pair {
  int first;
  double second;
  template <decltype(sizeof 0) I> constexpr auto get() const {
    if constexpr (I == 0) return first; else return second;
  }
};
template <> struct std::tuple_size<Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <> struct std::tuple_element<0, Pair> { using type = int; };
template <> struct std::tuple_element<1, Pair> { using type = double; };
constexpr bool check() {
  auto [a, b] = Pair{1, 2.5};
  return a == 1 && b == 2.5;
}
static_assert(check());
Pair p{3, 4.5};
auto [c, d] = p;
static_assert(same<decltype(c), int>);
static_assert(same<decltype(d), double>);
