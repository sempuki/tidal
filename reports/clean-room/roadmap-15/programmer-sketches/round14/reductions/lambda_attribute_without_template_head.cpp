int main() {
  auto f = [] [[nodiscard]] (int x) { return x + 1; };
  return f(1) - 2;
}
