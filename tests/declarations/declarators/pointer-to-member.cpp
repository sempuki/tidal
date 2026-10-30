// tidal-rules: DCL-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int v; double w; };
int S::* pm = &S::v;
static_assert(same<decltype(&S::w), double S::*>);
