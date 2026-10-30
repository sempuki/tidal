// tidal-rules: CX-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr int n = 3;
static_assert(same<decltype(n), const int>);
int a[n];
