// tidal-rules: FN-15 OVL-1
// tidal-expect: run
// tidal-note: The declarations have different parameter types, so they declare three functions with one name. A call selects one of them by overload resolution.
int kind(int) { return 1; }
int kind(long) { return 2; }
int kind(int*) { return 3; }
struct Gauge {
  int read() & { return 4; }
  int read() && { return 5; }
  int peek() { return 6; }
  int peek() const { return 7; }
};
int main() {
  int v = 0;
  Gauge g;
  const Gauge& cg = g;
  if (kind(0) != 1 || kind(0L) != 2 || kind(&v) != 3) return 1;
  if (g.read() != 4 || Gauge{}.read() != 5) return 2;
  return g.peek() == 6 && cg.peek() == 7 ? 0 : 3;
}
