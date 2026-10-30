// tidal-rules: AUTO-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
const int ci = 0;
auto&& a = i;
auto&& b = ci;
auto&& c = 1;
auto&& d = static_cast<int&&>(i);
static_assert(same<decltype(a), int&>);
static_assert(same<decltype(b), const int&>);
static_assert(same<decltype(c), int&&>);
static_assert(same<decltype(d), int&&>);
