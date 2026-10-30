// tidal-rules: AUTO-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int ci = 1;
decltype(auto) a = ci;
static_assert(same<decltype(a), const int>);
