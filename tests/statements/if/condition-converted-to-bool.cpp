// tidal-rules: IF-3
// tidal-expect: run
struct Flag {
  bool v;
  explicit operator bool() const { return v; }
};
int main() {
  int hits = 0;
  int* null = nullptr;
  int x = 5;
  if (null) return 1;
  if (&x) ++hits;
  if (-3) ++hits;
  if (0.0) return 2;
  if (Flag{true}) ++hits;
  if (Flag{false}) return 3;
  return hits == 3 ? 0 : 4;
}
