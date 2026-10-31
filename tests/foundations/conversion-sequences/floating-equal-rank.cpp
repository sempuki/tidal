// tidal-rules: ICS-35
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: invalid suffix .f64. on floating constant
// tidal-note: PRIM-22: Clang has no f64 suffix (conditionally supported). Every conversion here has conversion rank. Only ICS-35 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(double);
Two take(int);
static_assert(same<decltype(take(1.0f64)), One>);
One wide(double);
Two wide(long double);
static_assert(same<decltype(wide(1.0f64)), One>);
