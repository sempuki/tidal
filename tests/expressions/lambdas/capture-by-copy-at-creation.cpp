// tidal-rules: LAM-4
// tidal-expect: run
int main() {
  int n = 1;
  auto g = [n] { return n; };
  n = 5;
  return g() == 1 ? 0 : 1;
}
