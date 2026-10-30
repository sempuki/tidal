// tidal-rules: SB-1
// tidal-expect: run
int main() {
  int a[2] = {1, 2};
  auto& [x, y] = a;
  x = 10;
  return a[0] == 10 && &y == &a[1] ? 0 : 1;
}
