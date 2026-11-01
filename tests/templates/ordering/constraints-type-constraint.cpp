// tidal-rules: ORD-13
// tidal-expect: accept
// tidal-note: The heads differ only in the concept on their type parameter. Strong subsumes Weak, so the first template wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> concept Weak = sizeof(T) > 0;
template <class T> concept Strong = Weak<T> && sizeof(T) > 1;
template <Strong T> One f(T);
template <Weak T> Two f(T);
static_assert(same<decltype(f(1)), One>);
