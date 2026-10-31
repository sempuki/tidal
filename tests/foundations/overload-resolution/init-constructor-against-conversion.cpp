// tidal-rules: OVL-14
// tidal-expect: accept
// tidal-note: Copy-initialization from another class takes the constructor and the conversion function. The constructor's parameter binds the lvalue without adding const, so its sequence is better (ICS-30).
struct Source;
struct Target {
  int from = 0;
  constexpr Target(Source&) : from(1) {}
  constexpr Target(int f) : from(f) {}
};
struct Source { constexpr operator Target() const { return Target(2); } };
constexpr int probe() {
  Source s;
  Target t = s;
  return t.from;
}
static_assert(probe() == 1);
