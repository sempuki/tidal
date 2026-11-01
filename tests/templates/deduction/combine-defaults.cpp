// tidal-rules: DED-23
// tidal-expect: accept
// tidal-note: U takes its default, which uses T. The trailing pack Rest gets no argument and is empty.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Box {};
template <class T, class U = Box<T>> Tag<U> wrap(T);
template <class T, class... Rest> constexpr int rest(T) { return sizeof...(Rest); }
static_assert(same<decltype(wrap(1)), Tag<Box<int>>>);
static_assert(rest(1) == 0);
