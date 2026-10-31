// tidal-rules: DTY-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
static_assert(same<decltype(static_cast<int&&>(i)), int&&>);
struct S { int m; };
static_assert(same<decltype(S{}.m), int>);
static_assert(same<decltype((S{}.m)), int&&>);
