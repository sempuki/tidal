// tidal-rules: DEFARG-3
// tidal-expect: run
int counter = 0;
int next() { return ++counter; }
int f(int v = next()) { return v; }
int main() {
  int a = f();
  int b = f();
  int c = f(10);
  return a == 1 && b == 2 && c == 10 && counter == 2 ? 0 : 1;
}
