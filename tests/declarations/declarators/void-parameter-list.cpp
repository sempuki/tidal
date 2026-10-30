// tidal-rules: DCL-9
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f(void);
void g();
static_assert(same<decltype(f), decltype(g)>);
