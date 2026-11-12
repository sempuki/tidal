int dbl(int x) { return 2 * x; }
int main() {
  auto f = &dbl;
  auto g = [&](auto x) { return f(x); };
  return g(3) - 6;
}
