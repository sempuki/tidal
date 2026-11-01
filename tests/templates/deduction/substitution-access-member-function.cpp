// tidal-rules: DED-26
// tidal-expect: accept
// tidal-note: The return type calls run. Locked::run is private, so substitution fails for Locked and the ellipsis function wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
class Locked { void run(); };
struct Free { void run(); };
template <class T> auto call(T t) -> decltype(t.run(), One{});
Two call(...);
static_assert(same<decltype(call(Free{})), One>);
static_assert(same<decltype(call(Locked{})), Two>);
