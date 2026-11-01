// tidal-rules: ADD-2 ADD-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Level { low = 1, high = 5 };
static_assert('a' + 1 == 'b');
static_assert(true + true == 2);
static_assert(high - low == 4);
static_assert(low + 0.5 > 1.0 || true);
static_assert(same<decltype(true + true), int>);
static_assert(same<decltype('a' - 'a'), int>);
static_assert(same<decltype(high + 1u), unsigned>);
static_assert(same<decltype(1.0f - 1), float>);
