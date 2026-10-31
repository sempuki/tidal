// tidal-rules: OVL-29 OVL-30
// tidal-expect: accept
// tidal-note: OVL-30 prefers operator F&&, which is more constrained. OVL-29 prefers operator F&, which returns the target's kind of reference. operator F& wins, so OVL-29 comes first.
using F = int();
constexpr int a() { return 1; }
constexpr int b() { return 2; }
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept SmallWhole = Small<T> && (T(1) / 2 == 0);
template <class T> struct Source {
  constexpr operator F&() const requires Small<T> { return a; }
  constexpr operator F&&() const requires SmallWhole<T> { return b; }
};
constexpr F& r = Source<int>{};
static_assert(r() == 1);
