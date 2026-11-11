// An attribute on a lambda's call operator, after the introducer.
int main() {
  auto f = [] [[nodiscard]] (int x) { return x + 1; };
  return f(-1);
}
