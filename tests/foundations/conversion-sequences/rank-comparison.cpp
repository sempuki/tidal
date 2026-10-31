// tidal-rules: ICS-27
// tidal-expect: accept
// tidal-note: Neither sequence is a subsequence of the other, and no tie-breaker applies to arithmetic conversions of different rank.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
short small = 1;
One take(int);
Two take(long);
static_assert(same<decltype(take(small)), One>);
One real(double);
Two real(long double);
static_assert(same<decltype(real(1.0f)), One>);
