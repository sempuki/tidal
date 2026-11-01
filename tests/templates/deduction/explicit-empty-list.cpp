// tidal-rules: DED-4
// tidal-expect: accept
// tidal-note: The plain function wins the plain call. The call with <> has only the template as candidate.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
One pick(int);
template <class T> Two pick(T);
static_assert(same<decltype(pick(1)), One>);
static_assert(same<decltype(pick<>(1)), Two>);
