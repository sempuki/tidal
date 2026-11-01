// tidal-rules: CTAD-14
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: The user guide gives Box<long>, which is no Box<T*>, so the alias drops it. The guide from the constructor gives Box<int*>. Without the alias the user guide wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <class T> Box(T*) -> Box<long>;
template <class T> using PointerBox = Box<T*>;
int n = 0;
PointerBox b(&n);
static_assert(same<decltype(b), Box<int*>>);
