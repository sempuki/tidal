// tidal-rules: OVL-14
// tidal-expect: accept
// tidal-note: A conversion function that returns a reference to const Box counts as a conversion to Box.
struct Box { int v = 5; };
constexpr Box shared{};
struct Source { constexpr operator const Box&() const { return shared; } };
constexpr int probe() {
  Box b = Source{};
  return b.v;
}
static_assert(probe() == 5);
