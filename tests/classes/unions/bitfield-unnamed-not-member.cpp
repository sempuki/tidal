// tidal-rules: BF-12 BF-7
// tidal-expect: accept
struct Flags {
  unsigned ready : 1;
  unsigned : 3;                        // padding, no member
  unsigned mode : 4;
  unsigned : 0;
  unsigned tail : 4;
  constexpr bool operator==(const Flags&) const = default;
};
constexpr Flags make() { Flags f{1, 5, 2}; return f; }     // three initializers
constexpr Flags copy(Flags f) { Flags g = f; g = f; return g; }
static_assert(copy(make()).mode == 5 && copy(make()).tail == 2);
static_assert(copy(make()) == make());
constexpr int sum() {
  auto [ready, mode, tail] = make();   // three names for three members
  return ready + mode + tail;
}
static_assert(sum() == 8);
