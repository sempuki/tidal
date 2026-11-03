// tidal-rules: CMP-5
// tidal-expect: run
// tidal-note: The NaN comes from 0.0 / 0.0 in double, which follows IEEE 754 on the matrix targets (MUL-11, MUL-20, Q-0004).
volatile double zero = 0.0;
int main() {
  double z = zero;   // one read of the volatile object (SEQ-4)
  double n = z / z;
  bool all_false = !(n < n) && !(n > n) && !(n <= n) && !(n >= n) && !(n == n);
  bool ne = (n != n) && (n != 1.0);
  return all_false && ne ? 0 : 1;
}
