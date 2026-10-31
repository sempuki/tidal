// tidal-rules: UN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int twice(int v) { return 2 * v; }
static_assert(same<decltype(+twice), int (*)(int)>);
