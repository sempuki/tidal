// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int f(int);
static_assert(same<decltype(f), int(int)>);
