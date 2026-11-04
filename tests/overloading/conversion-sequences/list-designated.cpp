// tidal-rules: ICS-18
// tidal-expect: accept
// tidal-note: Swapped has a sequence although its order differs, and it ties with Point, so the second argument decides. Other has no member x, so it has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x; int y; };
struct Swapped { int y; int x; };
struct Other { int z; };
One take(Point, int);
Two take(Swapped, long);
static_assert(same<decltype(take({.x = 1, .y = 2}, 0)), One>);
One pick(Point);
Two pick(Other);
static_assert(same<decltype(pick({.x = 1})), One>);
