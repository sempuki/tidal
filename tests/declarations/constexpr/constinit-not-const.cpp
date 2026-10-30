// tidal-rules: CINIT-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constinit int counter = 5;
static_assert(same<decltype(counter), int>);
