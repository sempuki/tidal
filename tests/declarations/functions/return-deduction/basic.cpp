// tidal-rules: RET-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a() { return 1; }
auto b(bool c) { if (c) return 1.5; return 2.5; }
auto c() {}
auto d() { return; }
void g();
auto e() { return g(); }
static_assert(same<decltype(a()), int>);
static_assert(same<decltype(b(true)), double>);
static_assert(same<decltype(c()), void>);
static_assert(same<decltype(d()), void>);
static_assert(same<decltype(e()), void>);
