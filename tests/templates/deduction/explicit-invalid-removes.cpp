// tidal-rules: DED-5
// tidal-expect: accept
// tidal-note: An explicit list that does not fit a template removes the template, also for an argument that narrows.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <int N> One size();
template <class T> Two size();
static_assert(same<decltype(size<int>()), Two>);
static_assert(same<decltype(size<3>()), One>);
template <unsigned char C> One code();
template <int C> Two code();
static_assert(same<decltype(code<300>()), Two>);
template <class T> One count(T);
template <class T, class U> Two count(T);
static_assert(same<decltype(count<int, int>(1)), Two>);
