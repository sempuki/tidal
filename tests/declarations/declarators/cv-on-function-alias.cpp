// tidal-rules: CV-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using F = void();
static_assert(same<const F, F>);
