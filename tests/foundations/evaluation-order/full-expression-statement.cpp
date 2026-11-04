// tidal-rules: VC-9
// tidal-expect: run
struct Probe {
  static inline int live = 0;
  Probe() { ++live; }
  ~Probe() { --live; }
};
int take(const Probe&) { return Probe::live; }
int main() {
  int during = 0;
  during = take(Probe());
  return during == 1 && Probe::live == 0 ? 0 : 1;
}
