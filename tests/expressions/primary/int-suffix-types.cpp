// tidal-rules: PRIM-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1u), unsigned int>);
static_assert(same<decltype(1l), long>);
static_assert(same<decltype(1ul), unsigned long>);
static_assert(same<decltype(1LL), long long>);
static_assert(same<decltype(1ull), unsigned long long>);
static_assert(same<decltype(1Lu), unsigned long>);
static_assert(same<decltype(4294967296u), unsigned long>);
