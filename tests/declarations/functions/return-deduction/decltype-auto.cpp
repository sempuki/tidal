// tidal-rules: RET-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
decltype(auto) a() { return x; }
decltype(auto) b() { return (x); }
decltype(auto) c() { return static_cast<int&&>(x); }
static_assert(same<decltype(a()), int>);
static_assert(same<decltype(b()), int&>);
static_assert(same<decltype(c()), int&&>);
