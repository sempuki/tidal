// tidal-rules: LAM-1
// tidal-expect: run
int main() {
  const int v = [] { int x = 2; return x * 3; }();
  return v == 6 ? 0 : 1;
}
