// tidal-rules: DCL-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using L = int&;
int x;
const L r = x;
static_assert(same<decltype(r), int&>);
