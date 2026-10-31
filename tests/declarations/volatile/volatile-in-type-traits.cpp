// tidal-rules: VOL-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
volatile int v;
static_assert(same<decltype(v), volatile int>);
static_assert(same<decltype((v)), volatile int&>);
static_assert(same<decltype(v + 1), int>);
