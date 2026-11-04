// tidal-rules: SB-5
// tidal-expect: accept
// tidal-note: The member get takes an int template parameter. The index of type std::size_t converts to int as a template argument, so the call is valid.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct Pair {
  int first, second;
  template <int I> constexpr int get() const { return I == 0 ? first : second; }
};
template <> struct std::tuple_size<Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, Pair> { using type = int; };
constexpr int run() { auto [a, b] = Pair{1, 2}; return a + b * 10; }
static_assert(run() == 21);
