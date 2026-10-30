// tidal-rules: LOG-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int* p = nullptr;
static_assert(same<decltype(1 && 2), bool>);
static_assert(same<decltype(!5), bool>);
static_assert(same<decltype(p || 0), bool>);
