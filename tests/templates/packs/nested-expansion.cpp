// tidal-rules: PACK-7
// tidal-expect: accept
// tidal-note: The inner T... is expanded inside each copy of the outer pattern, which expands the other T.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class... T> struct Each { using type = List<List<T, List<T...>>...>; };
static_assert(same<Each<int, char>::type, List<List<int, List<int, char>>, List<char, List<int, char>>>>);
template <class... T> constexpr int sum(T... x) { return (x + ... + 0); }
template <class... T> constexpr int cross(T... x) { return (sum(x, x...) + ... + 0); }
static_assert(cross(1, 2) == 9);   // sum(1, 1, 2) + sum(2, 1, 2)
