constexpr int half(int x) {
  [[assume(x % 2 == 0)]];
  return x / 2;
}
static_assert(half(8) == 4);
int main() { return half(10) - 5; }
