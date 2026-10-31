// tidal-rules: UNEV-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int m; };
using T = decltype(S::m);
auto n = sizeof(S::m);
static_assert(same<T, int>);
