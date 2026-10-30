// tidal-rules: SB-1
// tidal-expect: run
int main() {
  int a[2] = {1, 2};
  auto [x, y] = a;
  x = 10;
  return a[0] == 1 && x == 10 && y == 2 ? 0 : 1;
}
