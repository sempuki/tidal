// tidal-rules: THR-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(throw 1), void>);
int pick(bool b) { return b ? 1 : throw 2; }
