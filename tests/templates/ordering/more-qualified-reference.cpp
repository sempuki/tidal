// tidal-rules: ORD-11
// tidal-expect: accept
// tidal-note: Both parameters become const int&, so the sequences tie. After the references are removed, const T has more qualifiers than T and decides.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One f(T&);
template <class T> Two f(const T&);
const int c = 1;
static_assert(same<decltype(f(c)), Two>);
