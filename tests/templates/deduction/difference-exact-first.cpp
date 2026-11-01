// tidal-rules: DED-14
// tidal-expect: accept
// tidal-note: Base<int> matches Base<T> exactly, so its base Base<long> is never tried.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Base {};
template <> struct Base<int> : Base<long> {};
template <class T> Tag<T> up(Base<T>&);
Base<int> b;
static_assert(same<decltype(up(b)), Tag<int>>);
