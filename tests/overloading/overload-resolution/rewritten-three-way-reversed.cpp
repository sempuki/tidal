// tidal-rules: OVL-13 DCMP-6
// tidal-expect: accept
// tidal-note: Only a three-way comparison that takes a Point first exists. 1 <=> p uses it reversed and evaluates as 0 <=> (p <=> 1), which flips the result.
struct Diff { int v; };
constexpr int operator<=>(int, Diff d) { return -d.v; }
struct Point { int v; };
constexpr Diff operator<=>(Point p, int n) { return Diff{p.v - n}; }
struct Size {
  int v;
  constexpr Diff operator<=>(int n) const { return Diff{v - n}; }
};
static_assert((Point{5} <=> 1).v == 4);
static_assert((1 <=> Point{5}) == -4);
static_assert((1 <=> Size{5}) == -4);
