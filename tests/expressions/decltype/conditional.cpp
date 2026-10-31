// tidal-rules: DTY-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a = 0, b = 0;
static_assert(same<decltype(true ? a : b), int&>);
static_assert(same<decltype(true ? a : 1), int>);
