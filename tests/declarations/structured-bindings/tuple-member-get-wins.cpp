// tidal-rules: SB-5
// tidal-expect: accept
// tidal-note: Pair has a member template named get, so the call is e.get<i>(), and the free get in the same namespace takes no part.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair {
  int first, second;
  template <decltype(sizeof 0) I> constexpr int get() const { return 100 + int(I); }
};
template <decltype(sizeof 0) I> constexpr int get(const Pair&) { return 0; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
constexpr int run() { auto [a, b] = lib::Pair{1, 2}; return a + b; }
static_assert(run() == 201);
