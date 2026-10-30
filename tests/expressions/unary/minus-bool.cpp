// tidal-rules: UN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(-true), int>);
static_assert(-true == -1);
