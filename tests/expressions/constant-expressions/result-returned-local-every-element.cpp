// tidal-rules: CE-21
// tidal-expect: accept
// tidal-note: Write report W11. A default member initializer gives every element a value before the builder runs.
struct Table { int v[4] = {}; };
constexpr Table make() {
  Table t;
  for (int i = 0; i < 3; ++i) t.v[i] = i + 1;
  return t;
}
constexpr Table table = make();
static_assert(table.v[2] == 3 && table.v[3] == 0);
