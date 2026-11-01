// tidal-rules: MUL-7 MUL-6
// tidal-expect: run
volatile int values[] = {7, -7, 9, -9, 1, -1, 0};
volatile int divisors[] = {2, -2, 4, -4, 3, -3};
int main() {
  for (int a : values) {
    for (int b : divisors) {
      int q = a / b;
      int r = a % b;
      if (q * b + r != a) return 1;
      if (r != 0 && (r < 0) != (a < 0)) return 2;
      if (r >= (b < 0 ? -b : b) || -r >= (b < 0 ? -b : b)) return 3;
    }
  }
  return 0;
}
