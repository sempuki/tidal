// tidal-rules: ICS-19
// tidal-expect: accept
// tidal-note: A list to a class with constructors is user-defined. Each element can use its own user-defined conversion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Box { Box(int); };
One take(Box);
Two take(long);
static_assert(same<decltype(take({1})), Two>);
struct Num { Num(int); };
struct Pair { Pair(Num, Num); };
One put(Pair);
Two put(...);
static_assert(same<decltype(put({1, 2})), One>);
