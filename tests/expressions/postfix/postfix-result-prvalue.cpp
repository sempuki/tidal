// tidal-rules: PFX-13
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
static_assert(same<decltype((i++)), int>);
