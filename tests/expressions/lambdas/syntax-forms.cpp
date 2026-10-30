// tidal-rules: LAM-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a = [] { return 1; };
auto b = [](int x) { return x * 2; };
auto c = [](int x) -> long { return x; };
static_assert(same<decltype(a()), int>);
static_assert(same<decltype(c(1)), long>);
static_assert(b(3) == 6);
