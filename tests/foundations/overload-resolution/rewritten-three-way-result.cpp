// tidal-rules: OVL-13
// tidal-expect: accept
// tidal-note: x < y uses (x <=> y) < 0. y < x uses the swapped form 0 < (x <=> y). Here x <=> y is -1.
struct Low { int v; };
struct High {
  int v;
  constexpr int operator<=>(const Low& o) const { return v < o.v ? -1 : v == o.v ? 0 : 1; }
};
constexpr High h{1};
constexpr Low l{2};
static_assert(h < l);
static_assert(!(l < h));
static_assert(l > h);
