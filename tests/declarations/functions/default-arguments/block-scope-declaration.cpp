// tidal-rules: DEFARG-2
// tidal-expect: run
int f(int a) { return a; }
int g() {
  int f(int a = 7);
  return f();
}
int h();
int main() { return g() == 7 ? 0 : 1; }
