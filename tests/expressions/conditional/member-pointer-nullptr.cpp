// tidal-rules: COND-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int v; };
bool c = true;
static_assert(same<decltype(c ? &S::v : nullptr), int S::*>);
