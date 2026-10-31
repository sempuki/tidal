// tidal-rules: MPTR-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
static_assert(same<decltype(&S::a), int S::*>);
static_assert(same<decltype(&S::f), int (S::*)(int) const>);
