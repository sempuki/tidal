// tidal-rules: MUL-10 MUL-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(short) < sizeof(int));
constexpr short lowest = -32767 - 1;
constexpr short minus_one = -1;
static_assert(same<decltype(lowest / minus_one), int>);
static_assert(lowest / minus_one == 32768);
static_assert(lowest % minus_one == 0);
