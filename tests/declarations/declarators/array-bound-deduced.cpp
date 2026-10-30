// tidal-rules: DCL-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a[] = {1, 2, 3};
char s[] = "abc";
static_assert(same<decltype(a), int[3]>);
static_assert(same<decltype(s), char[4]>);
static_assert(sizeof(a) == 3 * sizeof(int));
