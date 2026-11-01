// tidal-rules: DED-2
// tidal-expect: accept
// tidal-note: The explicit arguments long and char are the first two elements of the pack. Deduction adds double from the third argument, and 1 and 2 convert.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class... Ts> Tag<void(Ts...)> all(Ts...);
static_assert(same<decltype(all<long, char>(1, 2, 3.0)), Tag<void(long, char, double)>>);
template <class R, class... Ts> Tag<R(Ts...)> make(Ts...);
static_assert(same<decltype(make<int>('a', 1L)), Tag<int(char, long)>>);
