// tidal-rules: LAM-3
// tidal-expect: run
int main() {
  int n = 0;
  auto g = [n]() mutable { return ++n; };
  g();
  int second = g();
  return second == 2 && n == 0 ? 0 : 1;
}
