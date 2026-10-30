// tidal-rules: CV-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int n = 3;
int a[n];
static_assert(same<decltype(a), int[3]>);
static_assert(n == 3);
