// tidal-rules: OVL-40
// tidal-expect: accept
// tidal-note: Direct-initialization of Target from Source takes the constructors of Target. The explicit conversion function makes the temporary that the copy constructor's parameter binds.
struct Target {
  int v = 0;
  constexpr Target(int x) : v(x) {}
};
struct Source { constexpr explicit operator Target() const { return Target(4); } };
constexpr int probe() {
  Source s;
  Target t(s);
  return t.v;
}
static_assert(probe() == 4);
