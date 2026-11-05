// tidal-rules: SB-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is not a function template
// tidal-note: Q-0395. Member lookup of get in Pair finds a static member variable template whose head starts with a value. Only a member function template makes the member form, so the call is get<i>(e), which finds lib::get. Clang and EDG accept. GCC takes the member form and rejects it.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair {
  int first, second;
  template <decltype(sizeof 0) I> static constexpr int get = 99;
};
template <decltype(sizeof 0) I> constexpr int get(const Pair& p) { return I == 0 ? p.first : p.second; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
constexpr int run() { auto [a, b] = lib::Pair{1, 2}; return a * 10 + b; }
static_assert(run() == 12);
