// tidal-rules: CTAD-11
// tidal-expect: accept
// tidal-note: A guide can be a template or not, can give defaults to its template parameters and its parameters, and T&& in a guide is a forwarding reference.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { template <class U> constexpr Box(U) {} template <class U, class V> constexpr Box(U, V) {} };
template <class T> Box(T&&) -> Box<T>;
int n = 0;
Box a(n);
Box b(1);
static_assert(same<decltype(a), Box<int&>> && same<decltype(b), Box<int>>);
Box(const char*) -> Box<long>;
Box c("text");
static_assert(same<decltype(c), Box<long>>);
template <class T, class U = short> Box(T*, T*) -> Box<U>;
Box d(&n, &n);
static_assert(same<decltype(d), Box<short>>);
template <class T> struct Item { template <class... U> constexpr Item(U...) {} };
template <class T> Item(T*, int = 0) -> Item<T>;
Item e(&n);
static_assert(same<decltype(e), Item<int>>);
