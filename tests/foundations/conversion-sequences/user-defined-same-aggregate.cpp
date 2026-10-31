// tidal-rules: ICS-25
// tidal-expect: accept
// tidal-note: Both sequences initialize the same class, so the second parts compare, and ICS-28 prefers the rvalue reference.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x, y; };
One take(const Point&);
Two take(Point&&);
static_assert(same<decltype(take({1, 2})), Two>);
struct Pair { Pair(int, int); };
One put(const Pair&);
Two put(Pair&&);
static_assert(same<decltype(put({1, 2})), Two>);
