// tidal-rules: RET-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto f() -> decltype(auto) { static int s; return (s); }
static_assert(same<decltype(f()), int&>);
