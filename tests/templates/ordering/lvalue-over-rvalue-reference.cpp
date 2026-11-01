// tidal-rules: ORD-10
// tidal-expect: accept
// tidal-note: Both parameters become int&, so the sequences tie. After the references are removed both types are T, and the lvalue reference decides.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One f(T&);
template <class T> Two f(T&&);
int n = 0;
static_assert(same<decltype(f(n)), One>);
