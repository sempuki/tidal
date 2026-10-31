// tidal-rules: LSUP-3
// tidal-expect: accept
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct Pair { int a; long b; template <decltype(sizeof 0) I> auto get() const { if constexpr (I == 0) return a; else return b; } };
template <> struct std::tuple_size<Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <> struct std::tuple_element<0, Pair> { using type = int; };
template <> struct std::tuple_element<1, Pair> { using type = long; };
int use() { auto [x, y] = Pair{1, 2}; return x + int(y); }
