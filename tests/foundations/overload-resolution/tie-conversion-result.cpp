// tidal-rules: OVL-26
// tidal-expect: accept
// tidal-note: Both conversion functions take the object the same way, so no argument decides, and no template, constraint, or rewrite is involved. The result that converts better to the target wins.
struct Source {
  constexpr operator int() const { return 1; }
  constexpr operator long() const { return 2; }
};
constexpr long a = Source{};
constexpr int b = Source{};
static_assert(a == 2);
static_assert(b == 1);
