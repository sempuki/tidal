// A recursive lambda through an explicit object parameter.
int main() {
  auto fib = [](this auto self, int n) -> int { return n < 2 ? n : self(n - 1) + self(n - 2); };
  return fib(10) == 55 ? 0 : 1;
}
