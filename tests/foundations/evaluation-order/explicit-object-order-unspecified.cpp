// tidal-rules: SEQ-1 CLS-13
// tidal-expect: run
// tidal-note: Q-0319. The copy of the object into the explicit object parameter self, the call next(), and the copy of the second argument happen in an unspecified order. Clang and GCC 16 make the copy into self first. GCC 14 calls next() first. The test accepts every order.
int count = 0;
struct Gauge {
  Gauge() = default;
  Gauge(const Gauge&) { ++count; }
  int read(this Gauge self, Gauge, int) { return 0; }
};
int next() { ++count; return 0; }
int main() {
  Gauge g;
  g.read(g, next());
  return count == 3 ? 0 : 1;
}
