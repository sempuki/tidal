// tidal-rules: CMP-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a = 1; double b = 2;
static_assert(same<decltype(a < b), bool>);
static_assert(same<decltype(a == b), bool>);
static_assert(same<decltype(a != 2), bool>);
