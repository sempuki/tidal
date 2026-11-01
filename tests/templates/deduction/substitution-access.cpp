// tidal-rules: DED-26
// tidal-expect: accept
// tidal-note: Secret::type is private, so substitution fails for Secret and the ellipsis function wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
class Secret { using type = int; };
struct Open { using type = int; };
template <class T> One get(T, typename T::type* = nullptr);
Two get(...);
static_assert(same<decltype(get(Open{})), One>);
static_assert(same<decltype(get(Secret{})), Two>);
