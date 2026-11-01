// tidal-rules: CMP-5
// tidal-expect: run
// tidal-note: The NaN comes from 0.0f / 0.0f and 0.0L / 0.0L, in types that follow IEEE 754 on the matrix targets (MUL-11, MUL-20).
volatile float zero_f = 0.0f;
volatile long double zero_l = 0.0L;
int main() {
  float n = zero_f / zero_f;
  long double m = zero_l / zero_l;
  if (n < 1.0f || n > 1.0f || n <= n || n >= n || n == n) return 1;
  if (!(n != n) || !(n != 0.0f)) return 2;
  if (m < m || m > m || m <= 0.0L || m >= 0.0L || m == m) return 3;
  if (!(m != m)) return 4;
  if (n == m || !(n != m)) return 5;
  return 0;
}
