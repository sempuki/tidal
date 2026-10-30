// tidal-rules: DCL-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using L = int&;
using R = int&&;
static_assert(same<L&, int&>);
static_assert(same<L&&, int&>);
static_assert(same<R&, int&>);
static_assert(same<R&&, int&&>);
