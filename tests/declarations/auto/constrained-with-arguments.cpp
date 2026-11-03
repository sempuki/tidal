// tidal-rules: AUTO-13
// tidal-expect: accept
// tidal-note: The concept takes a template argument after the deduced type, and the constrained placeholder appears in a new expression.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> concept Same = same<T, U>;
Same<long> auto x = 1L;
static_assert(same<decltype(x), long>);
Same<int> decltype(auto) y = 3;
static_assert(same<decltype(y), int>);
auto* p = new Same<double> auto(2.5);
