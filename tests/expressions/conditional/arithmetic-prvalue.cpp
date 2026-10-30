// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
long l = 0;
double d = 0;
static_assert(same<decltype(true ? i : l), long>);
static_assert(same<decltype(true ? i : d), double>);
static_assert(same<decltype(true ? 'a' : 'b'), char>);
static_assert(same<decltype(true ? (char)1 : (short)2), int>);
