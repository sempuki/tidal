// tidal-rules: ORD-13
// tidal-expect: accept
// tidal-note: The placeholder parameter adds an invented type parameter to the first template's head. The two heads then have one type parameter each, and the stronger constraint decides.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> concept Weak = sizeof(T) > 0;
template <class T> concept Strong = Weak<T> && sizeof(T) > 1;
One f(Strong auto);
template <Weak T> Two f(T);
static_assert(same<decltype(f(1)), One>);
