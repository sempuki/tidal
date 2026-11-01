// tidal-rules: CTAD-7
// tidal-expect: accept
// tidal-note: The copy candidate takes Box<T> and gives Box<T>, so a Box<int> initializer gives Box<int> in every form.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { T v; constexpr Box(T x) : v(x) {} };
constexpr Box<int> one(1);
Box a(one);
Box b = one;
Box c{one};
Box d = {one};
static_assert(same<decltype(a), Box<int>> && same<decltype(b), Box<int>>);
static_assert(same<decltype(c), Box<int>> && same<decltype(d), Box<int>>);
