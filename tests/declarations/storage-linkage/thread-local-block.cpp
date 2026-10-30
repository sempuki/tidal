// tidal-rules: STO-4
// tidal-expect: run
int inits = 0;
int make() { return ++inits; }
int f() {
  thread_local int value = make();
  return ++value;
}
int main() {
  int a = f();
  int b = f();
  return a == 2 && b == 3 && inits == 1 ? 0 : 1;
}
