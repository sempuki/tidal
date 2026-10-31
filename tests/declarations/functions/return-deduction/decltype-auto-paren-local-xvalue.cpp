// tidal-rules: RET-6
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
decltype(auto) f() { int x = 1; return (x); }
static_assert(same<decltype(f()), int&&>);
int g = 2;
decltype(auto) h() { return (g); }
static_assert(same<decltype(h()), int&>);
