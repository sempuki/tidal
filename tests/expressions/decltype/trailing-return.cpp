// tidal-rules: DTY-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto add(int a, long b) -> decltype(a + b) { return a + b; }
static_assert(same<decltype(add(1, 2)), long>);
