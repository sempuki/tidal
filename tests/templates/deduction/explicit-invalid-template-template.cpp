// tidal-rules: DED-5
// tidal-expect: accept
// tidal-note: A class template as the argument fits only the template template parameter, and int fits only the type parameter. The template that does not fit adds no candidate.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> struct Box {};
template <template <class> class W> One wrap();
template <class T> Two wrap();
static_assert(same<decltype(wrap<Box>()), One>);
static_assert(same<decltype(wrap<int>()), Two>);
