// tidal-rules: AUTO-9
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
auto a = 1, *p = &i, &r = i;
static_assert(same<decltype(a), int>);
static_assert(same<decltype(p), int*>);
static_assert(same<decltype(r), int&>);
