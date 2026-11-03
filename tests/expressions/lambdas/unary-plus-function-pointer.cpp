// tidal-rules: LAM-10 UN-1
// tidal-expect: accept
// tidal-note: Write report W12. Unary + converts a lambda without captures to a pointer to function first, then applies to the pointer.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto p = +[](int x) { return x * 2; };
static_assert(same<decltype(p), int (*)(int)>);
