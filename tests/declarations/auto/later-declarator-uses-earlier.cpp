// tidal-rules: AUTO-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a = 1, *p = &a;
static_assert(same<decltype(p), int*>);
