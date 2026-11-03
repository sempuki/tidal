// tidal-rules: PRIM-6
// tidal-expect: accept
// tidal-note: On the x86-64 Linux target, the code units of a multi-character literal fill an int from the high end, the last character in the lowest byte.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype('abcd'), int>);
static_assert('abc' == 0x616263);
static_assert('abcd' == 0x61626364);
