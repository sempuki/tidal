// tidal-rules: DED-8
// tidal-expect: accept
// tidal-note: An lvalue gives T an lvalue reference type, and any other argument gives the plain type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> fwd(T&&);
int n = 0;
const int c = 1;
static_assert(same<decltype(fwd(n)), Tag<int&>>);
static_assert(same<decltype(fwd(c)), Tag<const int&>>);
static_assert(same<decltype(fwd(1)), Tag<int>>);
static_assert(same<decltype(fwd(static_cast<int&&>(n))), Tag<int>>);
template <class T> constexpr bool binds_lvalue(T&& x) { return same<decltype(x), int&>; }
static_assert(binds_lvalue(n));
