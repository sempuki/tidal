// tidal-rules: MUL-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char c = 1;
short s = 1;
long l = 1;
unsigned u = 1;
float f = 1;
static_assert(same<decltype(c * c), int>);
static_assert(same<decltype(s / c), int>);
static_assert(same<decltype(s % l), long>);
static_assert(same<decltype(u * 1), unsigned>);
static_assert(same<decltype(f * 2), float>);
static_assert(same<decltype(f / 2.0), double>);
static_assert(same<decltype(l * 1.0f), float>);
static_assert(-1 * 1u == ~0u);
