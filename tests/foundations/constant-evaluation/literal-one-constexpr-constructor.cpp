// tidal-rules: CE-23
// tidal-expect: accept
// tidal-note: One constexpr constructor that is not a copy or move constructor makes Gauge a literal type, also with another constructor that is not constexpr.
struct Gauge {
  int level;
  Gauge(double v) : level(int(v)) {}
  constexpr Gauge(int v) : level(v) {}
};
template <Gauge G> struct Dial { static constexpr int level = G.level; };
static_assert(Dial<Gauge(3)>::level == 3);
