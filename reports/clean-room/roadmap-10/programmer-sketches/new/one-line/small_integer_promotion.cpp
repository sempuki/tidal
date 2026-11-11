// unsigned char operands promote to int, so the difference can be negative.
int main() {
  unsigned char a = 3, b = 5;
  auto d = a - b;
  static_assert(sizeof(d) == sizeof(int));
  unsigned u = 3u - 5u;
  return d == -2 && u == 4294967294u ? 0 : 1;
}
