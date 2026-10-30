// tidal-rules: CV-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using C = const int;
const C x = 1;
static_assert(same<decltype(x), const int>);
