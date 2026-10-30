// tidal-rules: PFX-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a[2];
static_assert(same<decltype((a[0])), int&>);
