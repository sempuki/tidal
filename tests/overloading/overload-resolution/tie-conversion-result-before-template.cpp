// tidal-rules: OVL-26 OVL-27
// tidal-expect: accept
// tidal-note: OVL-27 prefers operator long, a plain function. OVL-26 prefers the template, whose result is exactly int. The template wins, so OVL-26 comes first.
struct Source {
  constexpr operator long() const { return 1; }
  template <class T> constexpr operator T() const { return 2; }
};
constexpr int n = Source{};
static_assert(n == 2);
