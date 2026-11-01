// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-note: The name Box without arguments is deduced in a variable declaration, a static data member with an initializer, a new expression, a functional cast, and the type of a constant template parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { T v; constexpr Box(T x) : v(x) {} };
Box a(1);
Box b{2L};
Box c = 'c';
Box d = {1.0};
const Box e(1u);
static_assert(same<decltype(a), Box<int>> && same<decltype(b), Box<long>> && same<decltype(c), Box<char>>);
static_assert(same<decltype(d), Box<double>> && same<decltype(e), const Box<unsigned>>);
auto p = new Box(3);
auto q = new const Box{4};
static_assert(same<decltype(p), Box<int>*> && same<decltype(q), const Box<int>*>);
static_assert(same<decltype(Box(1)), Box<int>> && same<decltype(Box{1u}), Box<unsigned>>);
template <Box B> constexpr auto value = B.v;
static_assert(value<Box<int>(5)> == 5);
struct Holder { static constexpr Box member = Box<int>(1); static inline Box other = 2; };
static_assert(same<decltype(Holder::member), const Box<int>> && same<decltype(Holder::other), Box<int>>);
