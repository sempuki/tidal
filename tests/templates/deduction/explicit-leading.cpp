// tidal-rules: DED-2
// tidal-expect: accept
// tidal-note: The explicit arguments go to To and then From. Deduction gives the rest.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class To, class From> Tag<To(From)> convert(From);
static_assert(same<decltype(convert<long>(1)), Tag<long(int)>>);
static_assert(same<decltype(convert<long, char>(1)), Tag<long(char)>>);
template <class... Ts> Tag<void(Ts...)> all(Ts...);
static_assert(same<decltype(all<long>(1, 'c')), Tag<void(long, char)>>);
