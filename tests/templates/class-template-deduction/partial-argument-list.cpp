// tidal-rules: CTAD-4
// tidal-expect: accept
// tidal-note: Pair<int> has an argument list, so no deduction takes place. U takes its default, int, and 2.5 converts to int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U = T> struct Pair { U second; constexpr Pair(T, U u) : second(u) {} };
constexpr Pair<int> p(1, 2.5);
static_assert(same<decltype(p), const Pair<int, int>> && p.second == 2);
