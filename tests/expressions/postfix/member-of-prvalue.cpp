// tidal-rules: PFX-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int v; };
S make();
static_assert(same<decltype((make().v)), int&&>);
