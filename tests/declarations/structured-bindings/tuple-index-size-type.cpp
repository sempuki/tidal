// tidal-rules: SB-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0376. The index is a constant of type std::size_t, so the parameter I of the member get deduces that type. Clang and EDG accept. GCC deduces int and rejects the static_assert.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Pair {
  int first, second;
  template <auto I> constexpr int get() const {
    static_assert(same<decltype(I), decltype(sizeof 0)>);
    return I == 0 ? first : second;
  }
};
template <> struct std::tuple_size<Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, Pair> { using type = int; };
constexpr int run() { auto [a, b] = Pair{1, 2}; return a + b * 10; }
static_assert(run() == 21);
