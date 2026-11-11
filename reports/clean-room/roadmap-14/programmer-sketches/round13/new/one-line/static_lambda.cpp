// A lambda with a static call operator, called and converted to a pointer.
int main() {
  auto twice = [](int x) static { return 2 * x; };
  int (*f)(int) = twice;
  return twice(3) + f(4) - 14;
}
