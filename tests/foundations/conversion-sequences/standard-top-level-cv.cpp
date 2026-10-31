// tidal-rules: ICS-1
// tidal-expect: accept
// tidal-note: The const on the argument and on the parameter of keep is not a step, so int to int is the identity.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(int);
Two take(long);
const int limit = 4;
static_assert(same<decltype(take(limit)), One>);
One keep(const int);
static_assert(same<decltype(keep(5)), One>);
