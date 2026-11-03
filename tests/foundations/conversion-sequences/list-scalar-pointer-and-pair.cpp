// tidal-rules: ICS-15
// tidal-expect: accept
// tidal-note: An empty list to a pointer is the identity. One float element to double is that element's promotion. A list of two elements has no sequence to int, so the class is chosen.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Pair { Pair(int, int); };
One take(int*);
Two take(Pair);
static_assert(same<decltype(take({})), One>);
One real(double);
Two real(long);
static_assert(same<decltype(real({1.5f})), One>);
One two(int);
Two two(Pair);
static_assert(same<decltype(two({1, 2})), Two>);
