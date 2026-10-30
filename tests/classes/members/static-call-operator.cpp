// tidal-rules: CLS-15
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct Twice {
  static int operator()(int x) { return x * 2; }
};
struct Grid {
  static int operator[](int r, int c) { return r * 10 + c; }
};
int main() {
  Twice t;
  Grid g;
  return t(3) == 6 && g[1, 2] == 12 ? 0 : 1;
}
