// tidal-rules: DCL-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a[][2] = {{1, 2}, {3, 4}, {5, 6}};
static_assert(same<decltype(a), int[3][2]>);
static_assert(same<decltype(a[0]), int (&)[2]>);
