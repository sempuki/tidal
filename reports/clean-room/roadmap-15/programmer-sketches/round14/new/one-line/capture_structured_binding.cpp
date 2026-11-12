struct P { int x, y; };
int main() {
  auto [a, b] = P{3, 4};
  auto f = [a, &b] { b += a; return a * b; };
  int r = f();
  return r - 21 + b - 7;
}
