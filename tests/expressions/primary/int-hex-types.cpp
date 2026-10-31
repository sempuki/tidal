// tidal-rules: PRIM-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(0x7fffffff), int>);
static_assert(same<decltype(0x80000000), unsigned int>);
static_assert(same<decltype(0xffffffffffffffff), unsigned long>);
static_assert(same<decltype(017777777777), int>);
static_assert(same<decltype(020000000000), unsigned int>);
