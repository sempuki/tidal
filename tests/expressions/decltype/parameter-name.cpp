// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f(const int& p, int v) { static_assert(same<decltype(p), const int&>); static_assert(same<decltype(v), int>); }
