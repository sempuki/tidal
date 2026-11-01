// tidal-rules: CTAD-20
// tidal-expect: accept
// tidal-note: The user guide takes Box<T> by value, as the copy candidate does. The user guide wins, so the result wraps: Box<Box<int>>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <class T> Box(Box<T>) -> Box<Box<T>>;
constexpr Box<int> one(1);
Box two(one);
static_assert(same<decltype(two), Box<Box<int>>>);
