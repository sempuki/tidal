// tidal-rules: LAM-14
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using A = decltype([] { return 1; });
using B = decltype([] { return 1; });
static_assert(!same<A, B>);
static_assert(sizeof([] {}) > 0);
