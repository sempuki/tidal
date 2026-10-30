// tidal-rules: STO-2
// tidal-expect: run
int inits = 0;
int make() { return ++inits * 10; }
int next() {
  static int counter = make();
  return ++counter;
}
int main() {
  int a = next();
  int b = next();
  int c = next();
  return a == 11 && b == 12 && c == 13 && inits == 1 ? 0 : 1;
}
