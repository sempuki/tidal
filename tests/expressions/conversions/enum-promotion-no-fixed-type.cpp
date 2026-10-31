// tidal-rules: CONV-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Fits { f1 = 1, f2 = 0x7fffffff };
enum NeedsUnsigned { u1 = 0xffffffff };
enum NeedsLong { n1 = -1, n2 = 0x80000000 };
static_assert(same<decltype(+f1), int>);
static_assert(same<decltype(+u1), unsigned int>);
static_assert(same<decltype(+n1), long>);
