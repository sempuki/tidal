// tidal-rules: UN-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i;
static_assert(same<decltype((++i)), int&>);
