// tidal-rules: DTY-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
struct S { int m; } s;
static_assert(same<decltype((i)), int&>);
static_assert(same<decltype((s.m)), int&>);
static_assert(same<decltype(*&i), int&>);
static_assert(same<decltype(i = 1), int&>);
static_assert(same<decltype(++i), int&>);
