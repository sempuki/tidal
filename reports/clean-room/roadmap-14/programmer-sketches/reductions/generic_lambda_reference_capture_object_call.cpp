struct Dbl { constexpr int operator()(int x) const { return 2 * x; } };
int main() {
  Dbl f;
  auto g = [&](auto x) { return f(x); };
  return g(3) - 6;
}
