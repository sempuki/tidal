// tidal-rules: LAM-15
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto id = [](auto x) { return x; };
static_assert(same<decltype(id(1)), int>);
static_assert(same<decltype(id(1.5)), double>);
