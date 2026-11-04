// tidal-rules: ICS-10
// tidal-expect: accept
// tidal-note: Pair's copy and move constructors would need a user-defined sequence from the inner list, so Pair has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Pair { Pair(int, int); };
struct Point { int x, y; };
struct Holder { Point p; };
One take(Pair);
Two take(Holder);
static_assert(same<decltype(take({{1, 2}})), Two>);
