// tidal-rules: RET-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
const int cx = 0;
auto a() { return x; }
auto b() -> auto& { return x; }
auto& c() { return cx; }
auto d() { return cx; }
static_assert(same<decltype(a()), int>);
static_assert(same<decltype(b()), int&>);
static_assert(same<decltype(c()), const int&>);
static_assert(same<decltype(d()), int>);
