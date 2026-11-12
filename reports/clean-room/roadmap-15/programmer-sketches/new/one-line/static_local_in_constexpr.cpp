constexpr int digit(int i) {
  static constexpr int table[] = {3, 1, 4, 1, 5};
  return table[i];
}
static_assert(digit(2) == 4);
int main() { return digit(4) - 5; }
