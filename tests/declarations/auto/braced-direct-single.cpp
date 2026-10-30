// tidal-rules: AUTO-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a{1};
auto b{1.5};
static_assert(same<decltype(a), int>);
static_assert(same<decltype(b), double>);
