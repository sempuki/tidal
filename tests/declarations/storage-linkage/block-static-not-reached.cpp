// tidal-rules: STO-2
// tidal-expect: run
int inits = 0;
int make() { return ++inits; }
int f(bool reach) {
  if (!reach) return 0;
  static int value = make();
  return value;
}
int main() {
  f(false);
  f(false);
  if (inits != 0) return 1;
  f(true);
  return inits == 1 ? 0 : 2;
}
