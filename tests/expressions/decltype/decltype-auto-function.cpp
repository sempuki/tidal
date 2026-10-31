// tidal-rules: DTY-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int g;
decltype(auto) ref() { return (g); }
static_assert(same<decltype(ref()), int&>);
