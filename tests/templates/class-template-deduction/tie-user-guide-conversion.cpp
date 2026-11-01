// tidal-rules: CTAD-20
// tidal-expect: accept
// tidal-note: For a short argument, the guide from Box(T) is an exact match, and the user guide from int needs a promotion, so the better conversion wins before CTAD-20. For an int argument both are exact, and the user guide wins the tie.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
Box(int) -> Box<long>;
constexpr short small = 1;
Box a(small);
Box b(1);
static_assert(same<decltype(a), Box<short>> && same<decltype(b), Box<long>>);
