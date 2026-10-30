// tidal-rules: COND-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a = 0, b = 0;
bool c = true;
static_assert(same<decltype((c ? static_cast<int&&>(a) : static_cast<int&&>(b))), int&&>);
static_assert(same<decltype((c ? a : static_cast<int&&>(b))), int>);
int&& r = c ? static_cast<int&&>(a) : static_cast<int&&>(b);
