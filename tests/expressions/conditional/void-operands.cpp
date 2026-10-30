// tidal-rules: COND-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f();
void g();
static_assert(same<decltype(true ? f() : g()), void>);
