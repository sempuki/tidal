// tidal-rules: LOG-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
bool x = true, y = false;
static_assert(same<decltype((x && y)), bool>);
static_assert(same<decltype((x || y)), bool>);
static_assert(same<decltype((!x)), bool>);
