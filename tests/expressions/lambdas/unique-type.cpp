// tidal-rules: LAM-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a = [] { return 1; };
auto b = [] { return 1; };
static_assert(!same<decltype(a), decltype(b)>);
static_assert(!same<decltype([]{}), decltype([]{})>);
