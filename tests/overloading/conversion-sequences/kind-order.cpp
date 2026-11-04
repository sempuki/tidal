// tidal-rules: ICS-21
// tidal-expect: accept
// tidal-note: Each pair differs only in kind: a conversion against a user-defined conversion, and a user-defined conversion against the ellipsis.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Box { Box(int); };
One take(long);
Two take(Box);
static_assert(same<decltype(take(1)), One>);
One put(Box);
Two put(...);
static_assert(same<decltype(put(1)), One>);
One any(long);
Two any(...);
static_assert(same<decltype(any(1)), One>);
