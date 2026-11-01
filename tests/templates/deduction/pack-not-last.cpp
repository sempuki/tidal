// tidal-rules: DED-20
// tidal-expect: accept
// tidal-note: A pack before the last parameter deduces nothing. It is empty, or it takes its explicit arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class... Ts> Tag<void(Ts...)> lead(Ts..., int);
static_assert(same<decltype(lead(1)), Tag<void()>>);
static_assert(same<decltype(lead<char>('a', 1)), Tag<void(char)>>);
