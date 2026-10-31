// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void g() { int x = 0; [=] { static_assert(same<decltype(x), int>); static_assert(same<decltype((x)), const int&>); }(); }
