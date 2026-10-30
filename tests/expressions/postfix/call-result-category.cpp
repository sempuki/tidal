// tidal-rules: PFX-9
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int& l(); int&& x(); int p();
static_assert(same<decltype((l())), int&>);
static_assert(same<decltype((x())), int&&>);
static_assert(same<decltype((p())), int>);
