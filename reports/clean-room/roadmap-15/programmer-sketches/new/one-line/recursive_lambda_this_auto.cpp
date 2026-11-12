int main() {
  auto fib = [](this auto self, int n) -> int { return n < 2 ? n : self(n - 1) + self(n - 2); };
  static_assert([](this auto self, int n) -> int { return n < 1 ? 0 : n + self(n - 1); }(4) == 10);
  return fib(10) == 55 ? 0 : 1;
}
