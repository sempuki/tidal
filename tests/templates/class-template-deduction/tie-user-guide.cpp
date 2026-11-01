// tidal-rules: CTAD-20
// tidal-expect: accept
// tidal-note: The user guide and the guide from Box(T) take the same parameter, and both are templates. Ranking and partial ordering tie, and the user guide wins, so the type is Box<long>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <class T> Box(T) -> Box<long>;
Box b(1);
static_assert(same<decltype(b), Box<long>>);
