// tidal-rules: ICS-28 ICS-35
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: invalid suffix .f64. on floating constant
// tidal-error-edg: static assertion failed
// tidal-note: Q-0152. Both sequences have conversion rank. ICS-35 prefers double, of equal rank, and ICS-28 prefers the rvalue reference. ICS-35 comes first. EDG applies ICS-28 first. PRIM-22: Clang has no f64 suffix.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const double&);
Two take(long double&&);
static_assert(same<decltype(take(1.0f64)), One>);
