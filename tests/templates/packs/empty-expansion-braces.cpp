// tidal-rules: PACK-9
// tidal-expect: accept
// tidal-note: With no elements, int{a...} is int{}, which gives zero; f(1, a...) is f(1); and List<T...> is List<>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class... A> constexpr int braced(A... a) { return int{a...}; }
static_assert(braced() == 0 && braced(6) == 6);
constexpr int count(int first) { return first; }
constexpr int count(int first, int second) { return first + second; }
template <class... A> constexpr int lead(A... a) { return count(1, a...); }
static_assert(lead() == 1 && lead(2) == 3);
template <class... T> using Wrap = List<T...>;
static_assert(same<Wrap<>, List<>>);
