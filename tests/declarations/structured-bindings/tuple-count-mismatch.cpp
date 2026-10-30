// tidal-rules: SB-5
// tidal-expect: reject
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
Pair p{1, 2.0};
auto [a, b, c] = p;
