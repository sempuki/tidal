// tidal-rules: CV-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using A = int[2];
const A a = {1, 2};
static_assert(same<decltype(a), const int[2]>);
static_assert(same<const A, const int[2]>);
static_assert(same<decltype(a[0]), const int&>);
