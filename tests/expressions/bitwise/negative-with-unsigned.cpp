// tidal-rules: BIT-4 CONV-2
// tidal-expect: accept
// tidal-note: X17-9. The common type of int and unsigned is unsigned, so a negative int operand converts modulo 2^N before the bitwise operation.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

static_assert(same<decltype(-1 & 0xFFu), unsigned>);
static_assert((-1 & 0xFFu) == 255u);
static_assert((-1 | 0u) == ~0u);
static_assert((-2 ^ 0u) == ~0u - 1u);
static_assert((-1L & 0xFFul) == 255ul);
