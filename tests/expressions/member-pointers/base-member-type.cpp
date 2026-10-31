// tidal-rules: MPTR-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct B { int x; };
struct D : B { int y; };
static_assert(same<decltype(&D::x), int B::*>);
static_assert(same<decltype(&D::y), int D::*>);
