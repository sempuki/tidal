// tidal-rules: DED-34 TARG-21
// tidal-expect: accept
// tidal-note: The pack in the head takes every explicit argument, so none reaches the invented parameter of auto. Without a pack, the explicit arguments after the head reach the invented parameters.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> constexpr int count(auto x) {
  return (same<decltype(x), long> ? 10 : 0) + int(sizeof...(T));
}
template <class T> constexpr bool second_is_long(auto x) { return same<decltype(x), long>; }
static_assert(count<int, long>(1) == 2);
static_assert(count<int, long>(1L) == 12);
static_assert(second_is_long<int, long>(1));
