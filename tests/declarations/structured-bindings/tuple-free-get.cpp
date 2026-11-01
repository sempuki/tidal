// tidal-rules: SB-5
// tidal-expect: accept
// tidal-note: Pair has no member named get, so the call is get<i>(e), and argument-dependent lookup finds lib::get.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair { int first, second; };
template <decltype(sizeof 0) I> constexpr int get(const Pair& p) { return I == 0 ? p.first : p.second; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
constexpr int run() { auto [a, b] = lib::Pair{1, 2}; return a * 10 + b; }
static_assert(run() == 12);
