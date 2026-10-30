// tidal-rules: RFOR-3
// tidal-expect: run
struct P { int a; int b; };
int main() {
  P ps[] = {{1, 2}, {3, 4}};
  int sum = 0;
  for (auto [x, y] : ps) sum += x * y;
  return sum == 14 ? 0 : 1;
}
