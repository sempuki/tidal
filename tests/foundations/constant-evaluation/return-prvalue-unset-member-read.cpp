// tidal-rules: CE-22 VC-4
// tidal-expect: accept
// tidal-note: Q-0115. Decision 0059, item F2 (R10-4), with the round 9 writer probe r10-4-return-local-unset.json. A constexpr function returns a prvalue whose member b has no value. The prvalue initializes the result object with no copy, so nothing reads b, and the caller reads member a.
struct Pair {
  int a;
  int b;
  constexpr Pair() : a(1) {}
};
constexpr Pair make() { return Pair(); }
static_assert(make().a == 1);
constexpr int read() {
  Pair q = make();
  return q.a;
}
static_assert(read() == 1);
