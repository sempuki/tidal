// tidal-rules: DED-15
// tidal-expect: accept
// tidal-note: Base<int> and Base<char> both match, and Base<int> is derived from Base<char>, so Base<int> decides.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Base {};
template <> struct Base<int> : virtual Base<char> {};
struct Side : virtual Base<char> {};
struct Join : Base<int>, Side {};
template <class T> Tag<T> up(Base<T>&);
Join j;
static_assert(same<decltype(up(j)), Tag<int>>);
