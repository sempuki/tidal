// tidal-rules: CTAD-11
// tidal-expect: accept
// tidal-note: The guide is declared after the first use, so only the second use sees it.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { template <class U> constexpr Box(U) {} constexpr Box(T) {} };
Box a(1L);
template <class U> Box(U) -> Box<char>;
Box b(1L);
static_assert(same<decltype(a), Box<long>> && same<decltype(b), Box<char>>);
