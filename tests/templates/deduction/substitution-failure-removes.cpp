// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: For int, T::size_type is not valid, so the template adds no candidate and the ellipsis function wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
struct Vec { using size_type = int; };
template <class T> One size_of(T, typename T::size_type = 0);
Two size_of(...);
static_assert(same<decltype(size_of(Vec{})), One>);
static_assert(same<decltype(size_of(1)), Two>);
