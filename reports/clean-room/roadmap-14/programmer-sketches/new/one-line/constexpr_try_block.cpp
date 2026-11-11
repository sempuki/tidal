constexpr int safe_div(int a, int b) {
  try {
    if (b == 0) return 0;
    return a / b;
  } catch (...) {
    return -1;
  }
}
static_assert(safe_div(9, 3) == 3 && safe_div(1, 0) == 0);
int main() { return safe_div(8, 2) - 4; }
