// tidal-rules: OVL-23
// tidal-expect: accept
// tidal-note: Each call has one candidate where every argument, the object included, has a sequence. The better-ranked candidates have an argument without one.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Box { Box(int); };
One f(int&, long);
Two f(const long&, Box);
int n = 0;
static_assert(same<decltype(f(n, 1)), One>);
static_assert(same<decltype(f(1, 1)), Two>);
struct Item {
  One get(long) &;
  Two get(double) &&;
};
Item v;
static_assert(same<decltype(v.get(1)), One>);
static_assert(same<decltype(Item{}.get(1)), Two>);
