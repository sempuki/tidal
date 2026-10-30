// tidal-rules: DCL-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int *p, q;
int a, *b, c[2], (*d)[2], e(int), (*f)(int);
static_assert(same<decltype(p), int*>);
static_assert(same<decltype(q), int>);
static_assert(same<decltype(b), int*>);
static_assert(same<decltype(c), int[2]>);
static_assert(same<decltype(d), int (*)[2]>);
static_assert(same<decltype(e), int(int)>);
static_assert(same<decltype(f), int (*)(int)>);
