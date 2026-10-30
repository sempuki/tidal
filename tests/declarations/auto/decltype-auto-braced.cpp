// tidal-rules: AUTO-11
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
decltype(auto) a{1};
decltype(auto) b{(i)};
static_assert(same<decltype(a), int>);
static_assert(same<decltype(b), int&>);
