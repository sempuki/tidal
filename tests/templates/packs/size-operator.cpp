// tidal-rules: PACK-19
// tidal-expect: accept
// tidal-note: sizeof... counts the elements of each kind of pack. The result is a constant of the type of sizeof, and the pack needs no ellipsis after it.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class> struct One {};
template <template <class> class... W> constexpr auto templates = sizeof...(W);
static_assert(templates<One, One> == 2);
template <int... N> constexpr auto values = sizeof...(N);
static_assert(values<1, 2, 3> == 3);
template <class... T> constexpr bool typed = same<decltype(sizeof...(T)), decltype(sizeof 0)>;
static_assert(typed<> && typed<int>);
template <class... T> constexpr int bound() { int a[sizeof...(T) + 1]; return sizeof a / sizeof a[0]; }
static_assert(bound<int, int>() == 3);
template <class... T> constexpr auto parameters(T... x) { return sizeof...(x); }
static_assert(parameters(1, 'c') == 2);
template <class... T> constexpr auto captures(T... x) { return [...y = x] { return sizeof...(y); }(); }
static_assert(captures(1, 2, 3) == 3);
