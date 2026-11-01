// tidal-rules: TARG-2
// tidal-expect: accept
// tidal-note: Member lookup of v after the dot finds the data member, so < is the less-than operator. The variable template v at namespace scope takes no part.
template <int A, int B> constexpr int v = A + B;
struct Gauge { int v = 0; };
constexpr int count(bool, bool) { return 2; }
constexpr int count(int) { return 1; }
constexpr int use() {
  Gauge g;
  return count(g.v < 1, 2 > (0));
}
static_assert(use() == 2);
