// tidal-rules: MUL-15
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(long) > sizeof(unsigned));
static_assert(same<decltype(-1L / 2u), long>);
static_assert(-1L / 2u == 0);
