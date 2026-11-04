// tidal-rules: OVL-26 OVL-30
// tidal-expect: accept
// tidal-note: Both conversion functions are plain functions with no parameters. OVL-30 prefers operator long, which is more constrained. OVL-26 prefers operator int, whose result is exactly int. operator int wins, so OVL-26 comes first.
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept SmallWhole = Small<T> && (T(1) / 2 == 0);
template <class T> struct Source {
  constexpr operator int() const requires Small<T> { return 1; }
  constexpr operator long() const requires SmallWhole<T> { return 2; }
};
constexpr int x = Source<int>{};
static_assert(x == 1);
