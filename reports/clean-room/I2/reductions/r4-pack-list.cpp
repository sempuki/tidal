template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class... A> int f(List<A...>);
static_assert(same<decltype(f(List<>{})), int>);
