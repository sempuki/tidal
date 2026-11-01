// tidal-rules: DED-18
// tidal-expect: accept
// tidal-note: The list deduces nothing, T comes from the first argument, and the list then initializes the second parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Box { T value; };
template <class T> Tag<T> put(T, Box<T>);
static_assert(same<decltype(put(1, {2})), Tag<int>>);
