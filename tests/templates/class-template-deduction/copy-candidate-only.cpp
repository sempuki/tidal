// tidal-rules: CTAD-7
// tidal-expect: accept
// tidal-note: The guide from the constructor template cannot give T. The copy candidate takes Box<int> and gives Box<int>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { template <class U> constexpr Box(U) {} };
constexpr Box<int> one(5);
Box two(one);
static_assert(same<decltype(two), Box<int>>);
