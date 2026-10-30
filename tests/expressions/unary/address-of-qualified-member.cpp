// tidal-rules: UN-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int m; int g(); };
static_assert(same<decltype(&S::m), int S::*>);
static_assert(same<decltype(&S::g), int (S::*)()>);
