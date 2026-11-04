// tidal-rules: VC-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int&& r = 1;
static_assert(same<decltype((r)), int&>);
