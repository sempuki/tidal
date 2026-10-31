// tidal-rules: CONV-19
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
short a = 1;
char b = 2;
static_assert(same<decltype(a + b), int>);
static_assert(same<decltype(true + true), int>);
