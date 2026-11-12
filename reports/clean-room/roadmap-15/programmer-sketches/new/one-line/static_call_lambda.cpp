int main() {
  auto twice = [](int x) static { return x * 2; };
  int (*fp)(int) = twice;
  return twice(3) + fp(4) == 14 ? 0 : 1;
}
