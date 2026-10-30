// tidal-rules: VC-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int f();
static_assert(same<decltype(f()), int>);
