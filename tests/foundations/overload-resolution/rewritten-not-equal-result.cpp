// tidal-rules: OVL-13
// tidal-expect: accept
// tidal-note: a != b uses !(a == b), and b != a uses !(a == b) with the operands swapped back.
struct Left { int v; };
struct Right {
  int v;
  constexpr bool operator==(const Left& o) const { return v == o.v; }
};
constexpr Right r{1};
constexpr Left l{2};
static_assert(r != l);
static_assert(l != r);
static_assert(!(Right{2} != Left{2}));
