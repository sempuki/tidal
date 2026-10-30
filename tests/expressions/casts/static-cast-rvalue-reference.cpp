// tidal-rules: CAST-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
static_assert(same<decltype(static_cast<int&&>(x)), int&&>);
