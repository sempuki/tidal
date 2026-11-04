// tidal-rules: OVL-21
// tidal-expect: accept
// tidal-note: The callee is the overload set itself, with no implied object argument. Resolution then runs as for a call by name.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One pick(int);
Two pick(long);
static_assert(same<decltype((&pick)(1)), One>);
static_assert(same<decltype((pick)(1L)), Two>);
