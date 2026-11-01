// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-note: Lists that do not bind to their element make a temporary. A short element converts to int, so int&& binds the temporary. A designated list makes a temporary Point.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x; int y; };
struct Single { int x; };
short s = 0;
One take(int&&);
Two take(long);
static_assert(same<decltype(take({s})), One>);
One move(int&&);
Two move(const int&);
static_assert(same<decltype(move({1})), One>);
One put(const Point&, int);
Two put(Single, long);
static_assert(same<decltype(put({.x = 1}, 0)), One>);
