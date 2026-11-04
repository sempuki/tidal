// tidal-rules: CONV-20 CONV-19 SHF-4
// tidal-expect: accept
// tidal-note: X17-4. The bitwise operators give their operands the common type of CONV-19, as the arithmetic operators do. A shift promotes each operand alone, so the left operand decides the type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

static_assert(same<decltype(1 & 1u), unsigned>);
static_assert(same<decltype(1 | 1L), long>);
static_assert(same<decltype(1 ^ 1ull), unsigned long long>);
static_assert((-1 & 3u) == 3u);
static_assert((-1 | 0u) == ~0u);
static_assert(same<decltype(1 << 1L), int>);
static_assert(same<decltype(1 << 1ull), int>);
static_assert(same<decltype(1u << 1), unsigned>);
static_assert(same<decltype(1L >> 1), long>);
