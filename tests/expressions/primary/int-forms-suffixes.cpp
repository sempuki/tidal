// tidal-rules: PRIM-3
// tidal-expect: accept
// tidal-note: The prefixes in upper case, and the suffix letters in both cases and both orders.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(0XfF == 255 && 0B101 == 5 && 0777 == 511);
static_assert(same<decltype(1uL), unsigned long>);
static_assert(same<decltype(1Lu), unsigned long>);
static_assert(same<decltype(1LU), unsigned long>);
static_assert(same<decltype(0x10ull), unsigned long long>);
static_assert(same<decltype(0b1LLU), unsigned long long>);
static_assert(same<decltype(07ll), long long>);
static_assert(0x10ull == 16 && 07ll == 7);
