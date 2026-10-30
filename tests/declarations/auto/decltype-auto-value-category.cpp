// tidal-rules: AUTO-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
decltype(auto) a = i;
decltype(auto) b = (i);
decltype(auto) c = static_cast<int&&>(i);
decltype(auto) d = 1;
static_assert(same<decltype(a), int>);
static_assert(same<decltype(b), int&>);
static_assert(same<decltype(c), int&&>);
static_assert(same<decltype(d), int>);
