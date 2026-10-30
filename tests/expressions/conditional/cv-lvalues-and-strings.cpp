// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 1;
const int cx = 2;
static_assert(same<decltype(true ? x : cx), const int&>);
static_assert(same<decltype(true ? "ab" : "c"), const char*>);
