// tidal-rules: DED-9
// tidal-expect: accept
// tidal-note: A parameter whose default argument the call uses takes no part, so its type gives T no second value.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> mix(T, T = 0L);
static_assert(same<decltype(mix(1)), Tag<int>>);
static_assert(same<decltype(mix(1, 2)), Tag<int>>);
