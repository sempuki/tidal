// tidal-rules: DCL-7
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int (*f())[3];
int (*g())(int);
static_assert(same<decltype(f), int (*())[3]>);
auto h() -> int (*)(int);
static_assert(same<decltype(h), decltype(g)>);
