int main() {
  int k = 2;
  auto g = [&](auto x) { return k * x; };
  return g(3) - 6;
}
