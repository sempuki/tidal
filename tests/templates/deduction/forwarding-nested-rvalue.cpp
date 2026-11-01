// tidal-rules: DED-8
// tidal-expect: accept
// tidal-note: The nested T&& matches int&& as an rvalue reference, and T is int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class T> Tag<T> take(void (*)(T&&));
void sink(int&&);
static_assert(same<decltype(take(sink)), Tag<int>>);
