// tidal-rules: ICS-25
// tidal-expect: accept
// tidal-note: Different constructors, different conversion functions, and an aggregate against a constructor tie, so the second argument decides each call (overload resolution referenced).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Left { Left(int); };
struct Right { Right(int); };
namespace first { One take(Left, int); Two take(Right, long); }
namespace second { One take(Left, long); Two take(Right, int); }
static_assert(same<decltype(first::take(1, 0)), One>);
static_assert(same<decltype(second::take(1, 0)), Two>);
struct Source { operator int(); operator long(); };
namespace third { One put(int, int); Two put(long, long); }
namespace fourth { One put(int, long); Two put(long, int); }
static_assert(same<decltype(third::put(Source{}, 0)), One>);
static_assert(same<decltype(fourth::put(Source{}, 0)), Two>);
struct Point { int x, y; };
struct Pair { Pair(int, int); };
namespace fifth { One set(Point, int); Two set(Pair, long); }
namespace sixth { One set(Point, long); Two set(Pair, int); }
static_assert(same<decltype(fifth::set({1, 2}, 0)), One>);
static_assert(same<decltype(sixth::set({1, 2}, 0)), Two>);
