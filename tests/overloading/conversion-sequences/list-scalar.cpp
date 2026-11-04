// tidal-rules: ICS-15
// tidal-expect: accept
// tidal-note: An empty list gives the identity, one element gives that element's sequence, and a nested list has no sequence to int.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Box { Box(int); };
struct Empty { Empty(); };
One take(int);
Two take(Empty);
static_assert(same<decltype(take({})), One>);
One one(int);
Two one(long);
static_assert(same<decltype(one({1})), One>);
One put(int);
Two put(Box);
static_assert(same<decltype(put({{1}})), Two>);
