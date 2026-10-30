// tidal-rules: CMP-5
// tidal-expect: run
volatile double zero = 0.0;
int main() {
  double n = zero / zero;
  bool all_false = !(n < n) && !(n > n) && !(n <= n) && !(n >= n) && !(n == n);
  bool ne = (n != n) && (n != 1.0);
  return all_false && ne ? 0 : 1;
}
