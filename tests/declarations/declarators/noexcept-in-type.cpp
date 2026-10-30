// tidal-rules: DCL-14
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f() noexcept;
void g();
static_assert(!same<decltype(f), decltype(g)>);
