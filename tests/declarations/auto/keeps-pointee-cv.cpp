// tidal-rules: AUTO-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int ci = 1;
auto p = &ci;
static_assert(same<decltype(p), const int*>);
const int* const cp = &ci;
auto q = cp;
static_assert(same<decltype(q), const int*>);
