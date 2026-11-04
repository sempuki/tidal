// tidal-rules: VC-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
int val();
static_assert(same<decltype((42)), int>);
static_assert(same<decltype((val())), int>);
static_assert(same<decltype((x++)), int>);
static_assert(same<decltype((&x)), int*>);
static_assert(same<decltype((x + 1)), int>);
static_assert(same<decltype((nullptr)), decltype(nullptr)>);
