// tidal-rules: PACK-5
// tidal-expect: accept
// tidal-note: The pattern holds a lambda expression that captures p. Each copy is its own closure and returns the element of its position.
template <class... T> constexpr int sum(T... v) { return (v + ... + 0); }
template <class... T> constexpr int each(T... p) { return sum([p] { return p; }()...); }
static_assert(each(1, 2, 3) == 6);
template <class... T> constexpr int weighted(T... p) {
  int total = 0;
  int weight = 1;
  ((total += weight++ * [p] { return p; }()), ...);
  return total;
}
static_assert(weighted(5, 7) == 19);
