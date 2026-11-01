// tidal-rules: OPR-7
// tidal-expect: run
struct Trace {
  int v = 0;
  constexpr int mark(int d) { v = v * 10 + d; return d; }
};
struct Grid {
  constexpr int operator[](int a, int b, int c) const { return a + b + c; }
};
constexpr Grid& pick(Trace& t, Grid& g) { t.mark(1); return g; }
constexpr int order() {
  Trace t;
  Grid g;
  (void)pick(t, g)[t.mark(2), t.mark(3), t.mark(4)];
  return t.v;
}
static_assert(order() == 1234);
int main() { return order() == 1234 ? 0 : 1; }
