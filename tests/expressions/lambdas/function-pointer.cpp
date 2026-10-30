// tidal-rules: LAM-10
// tidal-expect: run
int main() {
  int (*p)(int) = [](int x) { return x + 1; };
  auto q = +[](int x) { return x * 3; };
  return p(1) == 2 && q(2) == 6 ? 0 : 1;
}
