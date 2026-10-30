// tidal-rules: STO-9
// tidal-expect: run-fail
// tidal-note: Gap review 2026-10-30.
int depth = 0;
int make();
int get() {
  static int value = make();
  return value;
}
int make() { return ++depth < 3 ? get() : 1; }
int main() { return get(); }
