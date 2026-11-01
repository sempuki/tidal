// tidal-rules: TPL-32
// tidal-expect: accept
// tidal-note: The top-level const and volatile of the parameter types are dropped. N is an int prvalue, and V gets the type long with no const.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <volatile int N> constexpr auto get() { return N; }
static_assert(get<5>() == 5 && same<decltype(get<5>()), int>);
template <const auto V> struct Hold { using type = decltype(V); };
static_assert(same<Hold<3L>::type, long>);
