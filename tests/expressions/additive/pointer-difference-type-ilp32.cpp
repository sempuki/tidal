// tidal-rules: ADD-12
// tidal-expect: accept
// tidal-flags: -m32
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(void*) == 4);
int a[2];
static_assert(same<decltype(&a[1] - &a[0]), int>);
