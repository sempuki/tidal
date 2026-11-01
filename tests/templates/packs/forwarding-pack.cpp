// tidal-rules: PACK-22
// tidal-expect: accept
// tidal-note: Each element of T&&... is a forwarding reference on its own: lvalues give lvalue reference types, and other arguments give the plain type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class... T> Tag<void(T...)> take(T&&...);
int n = 0;
const int c = 1;
static_assert(same<decltype(take(n, c, 1, static_cast<int&&>(n))), Tag<void(int&, const int&, int, int)>>);
