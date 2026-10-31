// tidal-rules: PRIM-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1uz), decltype(sizeof 0)>);
static_assert(sizeof(1z) == sizeof(decltype(sizeof 0)));
static_assert(1z - 2z < 0);
