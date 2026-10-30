// tidal-rules: DEFARG-3
// tidal-expect: run
int value = 1;
int f(int v = value) { return v; }
int main() {
  value = 5;
  return f() == 5 ? 0 : 1;
}
