// tidal-rules: PACK-24
// tidal-expect: accept
// tidal-note: The reference collapses the same way through an alias, a template argument, and decltype. const on a reference type through an alias has no effect.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using LRef = int&;
using RRef = int&&;
static_assert(same<LRef&, int&> && same<LRef&&, int&> && same<RRef&, int&> && same<RRef&&, int&&>);
template <class T> using AddRvalue = T&&;
static_assert(same<AddRvalue<int&>, int&> && same<AddRvalue<int>, int&&>);
int n = 0;
int& rn = n;
static_assert(same<decltype(rn)&&, int&> && same<decltype((n))&&, int&>);
static_assert(same<decltype(static_cast<int&&>(n))&, int&>);
static_assert(same<const LRef, int&>);
