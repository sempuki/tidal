// tidal-rules: CTAD-17
// tidal-expect: accept
// tidal-note: Copy initialization with = leaves out the explicit user guide, so the guide from Box(T) gives Box<int*>. Direct initialization uses the user guide.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { template <class U> constexpr Box(U) {} constexpr Box(T) {} };
template <class T> explicit Box(T*) -> Box<T>;
int n = 0;
Box a(&n);
Box b = &n;
static_assert(same<decltype(a), Box<int>> && same<decltype(b), Box<int*>>);
