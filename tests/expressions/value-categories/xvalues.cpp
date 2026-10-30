// tidal-rules: VC-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
int&& move_ref();
struct S { int m; };
S s;
S make();
static_assert(same<decltype((static_cast<int&&>(x))), int&&>);
static_assert(same<decltype((move_ref())), int&&>);
static_assert(same<decltype((static_cast<S&&>(s).m)), int&&>);
static_assert(same<decltype((make().m)), int&&>);
