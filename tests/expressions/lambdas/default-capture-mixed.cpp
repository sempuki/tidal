// tidal-rules: LAM-5
// tidal-expect: run
int main() {
  int a = 1, b = 2;
  auto g = [=, &b] { b = a; };
  auto h = [&, a] { return a + b; };
  g();
  return b == 1 && h() == 2 ? 0 : 1;
}
