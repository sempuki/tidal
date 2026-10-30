// tidal-rules: LAM-7
// tidal-expect: run
struct P { int a; int b; };
int main() {
  auto [x, y] = P{1, 2};
  auto g = [x, &y] { return x + y; };
  return g() == 3 ? 0 : 1;
}
