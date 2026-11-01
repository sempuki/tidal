// tidal-rules: PACK-27
// tidal-expect: accept
// tidal-note: In the return type, T and U have one and two elements, so substitution fails and the call chooses the other overload. With equal lengths the template is chosen.
template <class... T> struct List {};
template <class A, class B> struct Pair {};
template <class... T, class... U> constexpr auto zip(List<T...>, List<U...>) -> List<Pair<T, U>...> { return {}; }
constexpr int zip(...) { return 2; }
static_assert(zip(List<int>(), List<int, char>()) == 2);
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(zip(List<int>(), List<char>())), List<Pair<int, char>>>);
