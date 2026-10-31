// tidal-rules: ICS-8 LIST-14
// tidal-expect: reject
// tidal-error-clang: designators to be specified in declaration order
// tidal-error-gcc: designator order
// tidal-error-edg: out-of-order initializers
// tidal-note: Designator order does not stop the sequence to Swapped. The second argument chooses take(Swapped, int), and the order then fails.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x; int y; };
struct Swapped { int y; int x; };
One take(Point, long);
Two take(Swapped, int);
auto r = take({.x = 1, .y = 2}, 0);
