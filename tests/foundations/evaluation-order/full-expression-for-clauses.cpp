// tidal-rules: VC-9
// tidal-expect: run
struct Probe {
  static inline int live = 0;
  Probe() { ++live; }
  ~Probe() { --live; }
};
int take(const Probe&) { return Probe::live; }
int main() {
  int bad = 0;
  for (int i = take(Probe()); take(Probe()) == 1 && i < 3; i += take(Probe())) {
    if (Probe::live != 0) bad = 1;
  }
  return bad == 0 && Probe::live == 0 ? 0 : 1;
}
