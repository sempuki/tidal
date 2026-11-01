// tidal-rules: ORD-5
// tidal-expect: accept
// tidal-note: Only the parameters for which the call has arguments take part. The defaulted parameter and the ellipsis do not.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One f(T*, int = 0);
template <class T> Two f(T);
template <class T> One g(T*, ...);
template <class T> Two g(T);
int* p = nullptr;
static_assert(same<decltype(f(p)), One>);
static_assert(same<decltype(g(p)), One>);
