// tidal-rules: CLS-12
// tidal-expect: run
int f() {
  static int counter = 4;
  struct S { int get() { return counter; } };
  return S{}.get();
}
int main() { return f() == 4 ? 0 : 1; }
