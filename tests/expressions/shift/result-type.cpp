// tidal-rules: SHF-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char c = 1; short s = 1; unsigned u = 1; long l = 1;
static_assert(same<decltype(c << 1), int>);
static_assert(same<decltype(s >> l), int>);
static_assert(same<decltype(u << 1L), unsigned>);
static_assert(same<decltype(l << c), long>);
static_assert(same<decltype(1 << 2ULL), int>);
