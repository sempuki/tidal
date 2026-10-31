// tidal-rules: TYN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
typedef int Int;
using Long = long;
static_assert(same<Int, int>);
static_assert(same<Long, long>);
