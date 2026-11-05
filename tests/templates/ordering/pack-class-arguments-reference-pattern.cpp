// tidal-rules: ORD-4
// tidal-expect: accept
// tidal-note: A&... is not deduced from the invented pack of plain types in List<T, A...>, so List<T, A&...> is more specialized. In the first call the packs have no element.
template <class... T> struct List {};
template <class T, class... A> constexpr int f(List<T, A...>) { return 1; }
template <class T, class... A> constexpr int f(List<T, A&...>) { return 2; }
static_assert(f(List<int>{}) == 2);
static_assert(f(List<int, float&>{}) == 2);
static_assert(f(List<int, float>{}) == 1);
