// tidal-rules: AUTO-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr auto n = 1;
static_assert(same<decltype(n), const int>);
